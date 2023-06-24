use crate::{LinkObject, NonFunctional, Object};
use reqwest as request;

//#[macro_export]
//macro_rules! resolve_all {
//    ($property:expr) => {{
//        let nf: &mut $crate::NonFunctional<_> = &mut $property;
//        match nf {
//            $crate::NonFunctional::None => {
//                Result::<_, Box<dyn ::std::error::Error>>::Ok($crate::NonFunctional::None)
//            }
//            $crate::NonFunctional::One(one) => Ok($crate::NonFunctional::One(one.resolve().await?)),
//            $crate::NonFunctional::Many(many) => Ok($crate::NonFunctional::Many(
//                ::futures::future::try_join_all(many.iter_mut().map(|one| one.resolve())).await?,
//            )),
//        }
//    }};
//}
//
//#[macro_export]
//macro_rules! resolve_maybe {
//    ($property:expr) => {{
//        let maybe: &mut Option<_> = &mut $property;
//        match maybe {
//            None => Result::<_, Box<dyn ::std::error::Error>>::Ok(None),
//            Some(some) => Ok(Some(some.resolve().await?)),
//        }
//    }};
//}

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

pub trait Resolve {
    type Output;
    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'this>>;
}

impl Resolve for LinkObject {
    type Output = Result<(), Box<dyn std::error::Error>>;

    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'this>> {
        Box::pin(async move {
            tracing::debug!("resolve {}", std::any::type_name::<Self>());
            match self {
                LinkObject::Object(_) => {
                    Ok(())
                }
                LinkObject::Url(url) => {
                    *self = LinkObject::Object(Box::new(request_object(url).await?));
                    Ok(())
                }
            }
        })
    }
}

impl<T, Ok, Err> Resolve for Option<T>
where
    T: Resolve<Output = Result<Ok, Err>>,
{
    type Output = Result<(), Err>;

    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'this>> {
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

impl<T, Ok, Err> Resolve for NonFunctional<T>
where
    T: Resolve<Output = Result<Ok, Err>>,
{
    type Output = Result<(), Err>;

    fn resolve<'this>(
        &'this mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'this>> {
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
