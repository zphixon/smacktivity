use reqwest as request;
use smacktivity::{LinkObject, NonFunctional, Object};

pub async fn request_object(url: impl AsRef<str>) -> Result<Object, Box<dyn std::error::Error>> {
    let client = request::Client::new();
    Ok(client
        .get(url.as_ref())
        .header(
            "Accept",
            "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
        )
        .send()
        .await?
        .json::<Object>()
        .await?)
}

pub trait Resolved {
    fn resolved<'this>(
        &'this mut self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<&'this mut Box<Object>, Box<dyn std::error::Error>>,
                > + 'this,
        >,
    >;
}

impl Resolved for LinkObject {
    fn resolved<'this>(
        &'this mut self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<&'this mut Box<Object>, Box<dyn std::error::Error>>,
                > + 'this,
        >,
    > {
        Box::pin(async move {
            #[derive(Debug)]
            struct ResolvedError(String);
            impl std::error::Error for ResolvedError {}
            impl std::fmt::Display for ResolvedError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "calling LinkObject::resolved(): {}", self.0)
                }
            }

            self.resolve().await?;
            match self {
                LinkObject::Object(object) => Ok(object),
                LinkObject::Url(url) => Err(ResolvedError(format!(
                    "called resolve but wasn't resolved ({})",
                    url
                ))
                .into()),
            }
        })
    }
}

pub type ResolveOutput = Result<(), Box<dyn std::error::Error>>;

pub trait Resolve {
    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ResolveOutput> + 'this>>;
}

impl Resolve for LinkObject {
    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ResolveOutput> + 'this>> {
        Box::pin(async move {
            tracing::debug!("resolve {}", std::any::type_name::<Self>());
            match self {
                LinkObject::Object(_) => Ok(()),
                LinkObject::Url(url) => {
                    *self = LinkObject::Object(Box::new(request_object(url).await?));
                    Ok(())
                }
            }
        })
    }
}

impl<T> Resolve for Option<T>
where
    T: Resolve,
{
    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ResolveOutput> + 'this>> {
        Box::pin(async move {
            tracing::debug!("resolve {}", std::any::type_name::<Self>());
            match self {
                Option::None => Ok(()),
                Option::Some(some) => {
                    some.resolve().await?;
                    Ok(())
                }
            }
        })
    }
}

impl<T> Resolve for NonFunctional<T>
where
    T: Resolve,
{
    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ResolveOutput> + 'this>> {
        Box::pin(async move {
            tracing::debug!("resolve {}", std::any::type_name::<Self>());
            match self {
                NonFunctional::None => Ok(()),
                NonFunctional::One(one) => {
                    one.resolve().await?;
                    Ok(())
                }
                NonFunctional::Many(many) => {
                    futures::future::try_join_all(many.iter_mut().map(|item| item.resolve()))
                        .await?;
                    Ok(())
                }
            }
        })
    }
}
