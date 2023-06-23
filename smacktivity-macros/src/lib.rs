use std::collections::HashSet;

use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    parse::Parse, parse_macro_input, spanned::Spanned, token, Expr, Ident, LitStr, Token, Type,
    TypePath,
};

fn is_object(type_: &Type) -> bool {
    matches!(type_,
        Type::Path(TypePath { path, qself: None })
            if path.segments.len() == 1
                && path.segments.first().unwrap().ident.to_string() == "Object"
    )
}

struct TypeVariant {
    type_: Type,
    variant_name: Option<Ident>,
}

impl Parse for TypeVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_ = input.parse()?;
        let variant_name = if input.peek(token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            Some(content.parse()?)
        } else {
            None
        };

        Ok(TypeVariant {
            type_,
            variant_name,
        })
    }
}

impl ToTokens for TypeVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variant_name = self
            .variant_name
            .as_ref()
            .map(|ident| Ok(ident.to_string()))
            .unwrap_or(match &self.type_ {
                Type::Path(TypePath { path, .. }) => match path.segments.last() {
                    Some(last) => Ok(last.ident.to_string().to_upper_camel_case()),
                    _ => Err("type path must have at least one segment"),
                },
                _ => Err("must be a type path"),
            });

        let variant_name = match variant_name {
            Ok(variant_name) => Ident::new(&variant_name, self.type_.span()),
            Err(err) => {
                let lit = LitStr::new(err, self.type_.span());
                tokens.extend(quote::quote! { compile_error(#lit) });
                return;
            }
        };

        let the_type = &self.type_;
        let type_ = if is_object(&self.type_) {
            quote::quote! { Box<#the_type> }
        } else {
            quote::quote! { #the_type }
        };

        tokens.extend(quote::quote! {
            #variant_name(#type_)
        });
    }
}

struct Property {
    name: Ident,
    serializes_to: String,
    type_name: String,
    types: Vec<TypeVariant>,
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

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = Ident::new(&self.type_name, self.name.span());
        let items = match &self.types[..] {
            [variant] => {
                let type_ = &variant.type_;
                if is_object(type_) {
                    quote::quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize)]
                        #[serde(transparent)]
                        pub struct #ident(pub Box<#type_>);
                    }
                } else {
                    quote::quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize)]
                        #[serde(transparent)]
                        pub struct #ident(pub #type_);
                    }
                }
            }

            _ => {
                let variants = self
                .types
                .iter()
                .map(|type_| quote::quote! { #type_ });

                quote::quote! {
                    #[derive(Debug, serde::Serialize, serde::Deserialize)]
                    pub enum #ident {
                        #(#variants,)*
                    }
                }
            }
        };
        tokens.extend(items);
    }
}

struct Object {
    properties: Vec<Property>,
}

impl Parse for Object {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let properties = input
            .parse_terminated(Property::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Object { properties })
    }
}

impl ToTokens for Object {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Object { properties } = self;

        let fields = properties.iter().map(
            |Property {
                 name,
                 type_name,
                 required,
                 ..
             }| {
                let type_ = Ident::new(&type_name, name.span());

                if !*required {
                    quote::quote! {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub #name: Option<#type_>,
                    }
                } else {
                    quote::quote! { pub #name: #type_, }
                }
            },
        );

        let the_struct = quote::quote! {
            #[derive(serde::Serialize)]
            pub struct Object {
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
            impl Default for Object {
                fn default() -> Object {
                    Object {
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

        let object_name_lit = LitStr::new("Object", Span::call_site());
        let debug_impl = quote::quote! {
            impl ::std::fmt::Debug for Object {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let mut dbg = f.debug_struct(#object_name_lit);
                    #(#required_debugs)*
                    #(#optional_debugs)*
                    dbg.finish()
                }
            }
        };

        properties.iter().for_each(|prop| prop.to_tokens(tokens));
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
