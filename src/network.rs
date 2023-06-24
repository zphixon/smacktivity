use crate::{LinkObject, Object};

impl Object {
    #[async_recursion::async_recursion]
    pub async fn resolve(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        for actor in self.actor.iter_mut() {
            if let LinkObject::Url(url) = actor {
                tracing::trace!("requesting {}: {}", "actor", url);
                *actor = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for attachment in self.attachment.iter_mut() {
            if let LinkObject::Url(url) = attachment {
                tracing::trace!("requesting {}: {}", "attachment", url);
                *attachment = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for attributed_to in self.attributed_to.iter_mut() {
            if let LinkObject::Url(url) = attributed_to {
                tracing::trace!("requesting {}: {}", "attributed_to", url);
                *attributed_to = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for audience in self.audience.iter_mut() {
            if let LinkObject::Url(url) = audience {
                tracing::trace!("requesting {}: {}", "audience", url);
                *audience = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for bcc in self.bcc.iter_mut() {
            if let LinkObject::Url(url) = bcc {
                tracing::trace!("requesting {}: {}", "bcc", url);
                *bcc = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for bto in self.bto.iter_mut() {
            if let LinkObject::Url(url) = bto {
                tracing::trace!("requesting {}: {}", "bto", url);
                *bto = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for cc in self.cc.iter_mut() {
            if let LinkObject::Url(url) = cc {
                tracing::trace!("requesting {}: {}", "cc", url);
                *cc = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for context in self.context.iter_mut() {
            if let LinkObject::Url(url) = context {
                tracing::trace!("requesting {}: {}", "context", url);
                *context = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        if let Some(LinkObject::Url(url)) = self.current.as_ref() {
            tracing::trace!("requesting {}: {}", "current", url);
            self.current = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        if let Some(LinkObject::Url(url)) = self.first.as_ref() {
            tracing::trace!("requesting {}: {}", "first", url);
            self.first = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        for generator in self.generator.iter_mut() {
            if let LinkObject::Url(url) = generator {
                tracing::trace!("requesting {}: {}", "generator", url);
                *generator = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for icon in self.icon.iter_mut() {
            if let LinkObject::Url(url) = icon {
                tracing::trace!("requesting {}: {}", "icon", url);
                *icon = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for image in self.image.iter_mut() {
            if let LinkObject::Url(url) = image {
                tracing::trace!("requesting {}: {}", "image", url);
                *image = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for in_reply_to in self.in_reply_to.iter_mut() {
            if let LinkObject::Url(url) = in_reply_to {
                tracing::trace!("requesting {}: {}", "in_reply_to", url);
                *in_reply_to = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for instrument in self.instrument.iter_mut() {
            if let LinkObject::Url(url) = instrument {
                tracing::trace!("requesting {}: {}", "instrument", url);
                *instrument = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        if let Some(LinkObject::Url(url)) = self.last.as_ref() {
            tracing::trace!("requesting {}: {}", "last", url);
            self.last = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        for location in self.location.iter_mut() {
            if let LinkObject::Url(url) = location {
                tracing::trace!("requesting {}: {}", "location", url);
                *location = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for items in self.items.iter_mut() {
            if let LinkObject::Url(url) = items {
                tracing::trace!("requesting {}: {}", "items", url);
                *items = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for ordered_items in self.ordered_items.iter_mut() {
            if let LinkObject::Url(url) = ordered_items {
                tracing::trace!("requesting {}: {}", "ordered_items", url);
                *ordered_items = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for one_of in self.one_of.iter_mut() {
            if let LinkObject::Url(url) = one_of {
                tracing::trace!("requesting {}: {}", "one_of", url);
                *one_of = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for any_of in self.any_of.iter_mut() {
            if let LinkObject::Url(url) = any_of {
                tracing::trace!("requesting {}: {}", "any_of", url);
                *any_of = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for origin in self.origin.iter_mut() {
            if let LinkObject::Url(url) = origin {
                tracing::trace!("requesting {}: {}", "origin", url);
                *origin = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        //if let Some(LinkObject::Url(url)) = self.next.as_ref() {
        //    tracing::trace!("requesting {}: {}", "next", url);
        //    self.next = Some(LinkObject::Object(Box::new(
        //        reqwest::get::<url::Url>(url.clone())
        //            .await?
        //            .json::<Object>()
        //            .await?
        //            .resolve()
        //            .await?,
        //    )));
        //}

        for object in self.object.iter_mut() {
            if let LinkObject::Url(url) = object {
                tracing::trace!("requesting {}: {}", "object", url);
                *object = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        //if let Some(LinkObject::Url(url)) = self.prev.as_ref() {
        //    tracing::trace!("requesting {}: {}", "prev", url);
        //    self.prev = Some(LinkObject::Object(Box::new(
        //        reqwest::get::<url::Url>(url.clone())
        //            .await?
        //            .json::<Object>()
        //            .await?
        //            .resolve()
        //            .await?,
        //    )));
        //}

        for preview in self.preview.iter_mut() {
            if let LinkObject::Url(url) = preview {
                tracing::trace!("requesting {}: {}", "preview", url);
                *preview = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for result in self.result.iter_mut() {
            if let LinkObject::Url(url) = result {
                tracing::trace!("requesting {}: {}", "result", url);
                *result = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for replies in self.replies.iter_mut() {
            if let LinkObject::Url(url) = replies {
                tracing::trace!("requesting {}: {}", "replies", url);
                *replies = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for tag in self.tag.iter_mut() {
            if let LinkObject::Url(url) = tag {
                tracing::trace!("requesting {}: {}", "tag", url);
                *tag = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for target in self.target.iter_mut() {
            if let LinkObject::Url(url) = target {
                tracing::trace!("requesting {}: {}", "target", url);
                *target = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for to in self.to.iter_mut() {
            if let LinkObject::Url(url) = to {
                tracing::trace!("requesting {}: {}", "to", url);
                *to = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        //for my_url in self.url.iter_mut() {
        //    if let LinkObject::Url(url) = my_url {
        //        tracing::trace!("requesting {}: {}", "my_url", url);
        //        *my_url = LinkObject::Object(Box::new(
        //            reqwest::get::<url::Url>(url.clone())
        //                .await?
        //                .json::<Object>()
        //                .await?
        //                .resolve()
        //                .await?,
        //        ));
        //    }
        //}

        //if let Some(LinkObject::Url(url)) = self.part_of.as_ref() {
        //    tracing::trace!("requesting {}: {}", "part_of", url);
        //    self.part_of = Some(LinkObject::Object(Box::new(
        //        reqwest::get::<url::Url>(url.clone())
        //            .await?
        //            .json::<Object>()
        //            .await?
        //            .resolve()
        //            .await?,
        //    )));
        //}

        if let Some(LinkObject::Url(url)) = self.subject.as_ref() {
            tracing::trace!("requesting {}: {}", "subject", url);
            self.subject = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        for relationship in self.relationship.iter_mut() {
            if let LinkObject::Url(url) = relationship {
                tracing::trace!("requesting {}: {}", "relationship", url);
                *relationship = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        for former_type in self.former_type.iter_mut() {
            if let LinkObject::Url(url) = former_type {
                tracing::trace!("requesting {}: {}", "former_type", url);
                *former_type = LinkObject::Object(Box::new(
                    reqwest::get::<url::Url>(url.clone())
                        .await?
                        .json::<Object>()
                        .await?
                        .resolve()
                        .await?,
                ));
            }
        }

        if let Some(LinkObject::Url(url)) = self.inbox.as_ref() {
            tracing::trace!("requesting {}: {}", "inbox", url);
            self.inbox = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        if let Some(LinkObject::Url(url)) = self.outbox.as_ref() {
            tracing::trace!("requesting {}: {}", "outbox", url);
            self.outbox = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        if let Some(LinkObject::Url(url)) = self.following.as_ref() {
            tracing::trace!("requesting {}: {}", "following", url);
            self.following = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        if let Some(LinkObject::Url(url)) = self.followers.as_ref() {
            tracing::trace!("requesting {}: {}", "followers", url);
            self.followers = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        if let Some(LinkObject::Url(url)) = self.liked.as_ref() {
            tracing::trace!("requesting {}: {}", "liked", url);
            self.liked = Some(LinkObject::Object(Box::new(
                reqwest::get::<url::Url>(url.clone())
                    .await?
                    .json::<Object>()
                    .await?
                    .resolve()
                    .await?,
            )));
        }

        Ok(self)
    }
}
