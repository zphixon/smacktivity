use std::collections::HashSet;

use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, parse_macro_input, token, Expr, Ident, LitStr, Token, Type, TypePath};

struct Property {
    name: Ident,
    serializes_to: String,
    type_name: String,
    types: Vec<Type>,
    required: bool,
    default: Option<Expr>,
}

impl Parse for Property {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let type_name = name.to_string().to_upper_camel_case() + "Property";

        let required = if input.peek(Token![?]) {
            let _question: Token![?] = input.parse()?;
            false
        } else {
            true
        };

        let serializes_to = if input.peek(token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let literal: LitStr = content.parse()?;
            literal.value()
        } else {
            name.to_string().to_lower_camel_case()
        };

        let _colon: Token![:] = input.parse()?;
        let mut types = vec![input.parse()?];
        while input.peek(Token![|]) {
            let _bar: Token![|] = input.parse()?;
            types.push(input.parse()?);
        }

        let default = if required {
            let _eq: Token![=] = input.parse()?;
            Some(input.parse()?)
        } else {
            if input.peek(Token![=]) {
                let _eq: Token![=] = input.parse()?;
                Some(input.parse()?)
            } else {
                None
            }
        };

        Ok(Property {
            name,
            type_name,
            serializes_to,
            types,
            required,
            default,
        })
    }
}

struct Object {
    object_name: Ident,
    properties: Vec<Property>,
}

impl Parse for Object {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let object_name = input.parse()?;

        let content;
        syn::braced!(content in input);
        let properties = content
            .parse_terminated(Property::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Object {
            object_name,
            properties,
        })
    }
}

impl ToTokens for Object {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Object {
            object_name,
            properties,
        } = self;

        fn is_object(type_: &Type, object_name: &Ident) -> bool {
            let object_name = object_name.to_string();
            matches!(type_,
                Type::Path(TypePath { path, qself: None })
                    if path.segments.len() == 1
                        && path.segments.first().unwrap().ident.to_string() == object_name
            )
        }

        let property_types = properties.iter().map(
            |Property {
                 name,
                 type_name,
                 types,
                 ..
             }| {
                let ident = Ident::new(&type_name, name.span());
                match &types[..] {
                    [type_] => {
                        if is_object(type_, object_name) {
                            quote::quote! {
                                #[derive(Debug)]
                                pub struct #ident(pub Box<#type_>);
                            }
                        } else {
                            quote::quote! {
                                #[derive(Debug)]
                                pub struct #ident(pub #type_);
                            }
                        }
                    }

                    _ => {
                        let variant_names = match types
                            .iter()
                            .map(|type_| match type_ {
                                Type::Path(TypePath { path, .. }) => match path.segments.last() {
                                    Some(last) => {
                                        Ok((last.ident.to_string().to_upper_camel_case(), type_))
                                    }
                                    _ => Err("type path must have at least one segment"),
                                },
                                _ => Err("must be a type path"),
                            })
                            .collect::<Result<Vec<_>, _>>()
                        {
                            Ok(variant_names) => variant_names,
                            Err(err) => {
                                let lit = LitStr::new(err, name.span());
                                return quote::quote! { compile_error(#lit) };
                            }
                        };

                        let variants = variant_names
                            .into_iter()
                            .map(|(variant_name, type_)| {
                                let ident = Ident::new(&variant_name, name.span());
                                if is_object(type_, object_name) {
                                    quote::quote! { #ident(Box< #type_ >), }
                                } else {
                                    quote::quote! { #ident(#type_), }
                                }
                            })
                            .collect::<Vec<_>>();

                        quote::quote! {
                            #[derive(Debug)]
                            pub enum #ident {
                                #(#variants)*
                            }
                        }
                    }
                }
            },
        );

        let fields = properties.iter().map(
            |Property {
                 name,
                 type_name,
                 required,
                 ..
             }| {
                let type_ = Ident::new(&type_name, name.span());

                let type_ = if !*required {
                    quote::quote! { Option<#type_> }
                } else {
                    quote::quote! { #type_ }
                };

                quote::quote! { pub #name: #type_, }
            },
        );

        let the_struct = quote::quote! {
            pub struct #object_name {
                #(#fields)*
            }
        };

        let defaults = properties
            .iter()
            .map(|prop| {
                prop.default
                    .clone()
                    .map(|default| (prop.name.clone(), quote::quote! { #default }))
                    .unwrap_or_else(|| (prop.name.clone(), quote::quote! { None }))
            })
            .map(|(name, default_value)| {
                quote::quote! {
                    #name: #default_value,
                }
            });

        let default_impl = quote::quote! {
            impl Default for #object_name {
                fn default() -> #object_name {
                    #object_name {
                        #(#defaults)*
                    }
                }
            }
        };

        let required_debugs = properties.iter().filter(|prop| prop.required).map(
            |Property {
                 name,
                 serializes_to,
                 ..
             }| {
                let lit = LitStr::new(&serializes_to, name.span());
                quote::quote! { dbg.field(#lit, &self.#name); }
            },
        );

        let optional_debugs = properties.iter().filter(|prop| !prop.required).map(
            |Property {
                 name,
                 serializes_to,
                 ..
             }| {
                let lit = LitStr::new(&serializes_to, name.span());
                quote::quote! {
                    if let Some(field) = self.#name.as_ref() {
                        dbg.field(#lit, field);
                    }
                }
            },
        );

        let object_name_lit = LitStr::new(&object_name.to_string(), object_name.span());
        let debug_impl = quote::quote! {
            impl ::std::fmt::Debug for #object_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let mut dbg = f.debug_struct(#object_name_lit);
                    #(#required_debugs)*
                    #(#optional_debugs)*
                    dbg.finish()
                }
            }
        };

        tokens.extend(property_types.into_iter());
        tokens.extend(the_struct.into_token_stream());
        tokens.extend(default_impl.into_token_stream());
        tokens.extend(debug_impl.into_token_stream());
    }
}

#[proc_macro]
pub fn object(input: TokenStream) -> TokenStream {
    let obj = parse_macro_input!(input as Object);
    quote::quote! {
        #obj
    }
    .into_token_stream()
    .into()
}

struct Present {
    total: HashSet<Ident>,
    supplied: HashSet<Ident>,
}

impl Parse for Present {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let total = input
            .parse_terminated(Ident::parse, Token![,])?
            .into_iter()
            .collect();

        let _semi: Token![;] = input.parse()?;

        let supplied = input
            .parse_terminated(Ident::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Present { total, supplied })
    }
}

#[proc_macro]
pub fn missing_from(input: TokenStream) -> TokenStream {
    let Present { total, supplied } = parse_macro_input!(input as Present);
    let missing = total.difference(&supplied);
    let fields = missing.map(|none| {
        quote::quote! { #none: None, }
    });
    quote::quote! { #(#fields)* }.into_token_stream().into()
}
