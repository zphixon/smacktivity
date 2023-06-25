use heck::ToSnakeCase;

#[derive(argh::FromArgs)]
#[argh(description = "read an activitypub object from stdin and do something with it")]
struct Args {
    #[argh(option, description = "resolve and get property")]
    resolve: Option<String>,

    #[argh(switch, description = "use debug printing rather than JSON")]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Args = argh::from_env();

    let mut object = serde_json::from_reader::<_, smacktivity::Object>(std::io::stdin())?;

    use smacktivity_network::Resolved;

    if let Some(property) = args.resolve.as_ref() {
        match property.to_snake_case().as_str() {
            "actor" => {
                for actor in object.actor.iter_mut() {
                    let actor = actor.resolved().await?;
                    if args.debug {
                        println!("{:#?}", actor);
                    } else {
                        println!("{}", serde_json::to_string(&actor)?);
                    }
                }
            }
            "attachment" => {
                for attachment in object.attachment.iter_mut() {
                    let attachment = attachment.resolved().await?;
                    if args.debug {
                        println!("{:#?}", attachment);
                    } else {
                        println!("{}", serde_json::to_string(&attachment)?);
                    }
                }
            }
            "attributed_to" => {
                for attributed_to in object.attributed_to.iter_mut() {
                    let attributed_to = attributed_to.resolved().await?;
                    if args.debug {
                        println!("{:#?}", attributed_to);
                    } else {
                        println!("{}", serde_json::to_string(&attributed_to)?);
                    }
                }
            }
            "audience" => {
                for audience in object.audience.iter_mut() {
                    let audience = audience.resolved().await?;
                    if args.debug {
                        println!("{:#?}", audience);
                    } else {
                        println!("{}", serde_json::to_string(&audience)?);
                    }
                }
            }
            "bcc" => {
                for bcc in object.bcc.iter_mut() {
                    let bcc = bcc.resolved().await?;
                    if args.debug {
                        println!("{:#?}", bcc);
                    } else {
                        println!("{}", serde_json::to_string(&bcc)?);
                    }
                }
            }
            "bto" => {
                for bto in object.bto.iter_mut() {
                    let bto = bto.resolved().await?;
                    if args.debug {
                        println!("{:#?}", bto);
                    } else {
                        println!("{}", serde_json::to_string(&bto)?);
                    }
                }
            }
            "cc" => {
                for cc in object.cc.iter_mut() {
                    let cc = cc.resolved().await?;
                    if args.debug {
                        println!("{:#?}", cc);
                    } else {
                        println!("{}", serde_json::to_string(&cc)?);
                    }
                }
            }
            "context" => {
                for context in object.context.iter_mut() {
                    let context = context.resolved().await?;
                    if args.debug {
                        println!("{:#?}", context);
                    } else {
                        println!("{}", serde_json::to_string(&context)?);
                    }
                }
            }
            "current" => {
                for current in object.current.iter_mut() {
                    let current = current.resolved().await?;
                    if args.debug {
                        println!("{:#?}", current);
                    } else {
                        println!("{}", serde_json::to_string(&current)?);
                    }
                }
            }
            "first" => {
                for first in object.first.iter_mut() {
                    let first = first.resolved().await?;
                    if args.debug {
                        println!("{:#?}", first);
                    } else {
                        println!("{}", serde_json::to_string(&first)?);
                    }
                }
            }
            "generator" => {
                for generator in object.generator.iter_mut() {
                    let generator = generator.resolved().await?;
                    if args.debug {
                        println!("{:#?}", generator);
                    } else {
                        println!("{}", serde_json::to_string(&generator)?);
                    }
                }
            }
            "icon" => {
                for icon in object.icon.iter_mut() {
                    let icon = icon.resolved().await?;
                    if args.debug {
                        println!("{:#?}", icon);
                    } else {
                        println!("{}", serde_json::to_string(&icon)?);
                    }
                }
            }
            "image" => {
                for image in object.image.iter_mut() {
                    let image = image.resolved().await?;
                    if args.debug {
                        println!("{:#?}", image);
                    } else {
                        println!("{}", serde_json::to_string(&image)?);
                    }
                }
            }
            "in_reply_to" => {
                for in_reply_to in object.in_reply_to.iter_mut() {
                    let in_reply_to = in_reply_to.resolved().await?;
                    if args.debug {
                        println!("{:#?}", in_reply_to);
                    } else {
                        println!("{}", serde_json::to_string(&in_reply_to)?);
                    }
                }
            }
            "instrument" => {
                for instrument in object.instrument.iter_mut() {
                    let instrument = instrument.resolved().await?;
                    if args.debug {
                        println!("{:#?}", instrument);
                    } else {
                        println!("{}", serde_json::to_string(&instrument)?);
                    }
                }
            }
            "last" => {
                for last in object.last.iter_mut() {
                    let last = last.resolved().await?;
                    if args.debug {
                        println!("{:#?}", last);
                    } else {
                        println!("{}", serde_json::to_string(&last)?);
                    }
                }
            }
            "location" => {
                for location in object.location.iter_mut() {
                    let location = location.resolved().await?;
                    if args.debug {
                        println!("{:#?}", location);
                    } else {
                        println!("{}", serde_json::to_string(&location)?);
                    }
                }
            }
            "items" => {
                for items in object.items.iter_mut() {
                    let items = items.resolved().await?;
                    if args.debug {
                        println!("{:#?}", items);
                    } else {
                        println!("{}", serde_json::to_string(&items)?);
                    }
                }
            }
            "ordered_items" => {
                for ordered_items in object.ordered_items.iter_mut() {
                    let ordered_items = ordered_items.resolved().await?;
                    if args.debug {
                        println!("{:#?}", ordered_items);
                    } else {
                        println!("{}", serde_json::to_string(&ordered_items)?);
                    }
                }
            }
            "one_of" => {
                for one_of in object.one_of.iter_mut() {
                    let one_of = one_of.resolved().await?;
                    if args.debug {
                        println!("{:#?}", one_of);
                    } else {
                        println!("{}", serde_json::to_string(&one_of)?);
                    }
                }
            }
            "any_of" => {
                for any_of in object.any_of.iter_mut() {
                    let any_of = any_of.resolved().await?;
                    if args.debug {
                        println!("{:#?}", any_of);
                    } else {
                        println!("{}", serde_json::to_string(&any_of)?);
                    }
                }
            }
            "origin" => {
                for origin in object.origin.iter_mut() {
                    let origin = origin.resolved().await?;
                    if args.debug {
                        println!("{:#?}", origin);
                    } else {
                        println!("{}", serde_json::to_string(&origin)?);
                    }
                }
            }
            "next" => {
                for next in object.next.iter_mut() {
                    let next = next.resolved().await?;
                    if args.debug {
                        println!("{:#?}", next);
                    } else {
                        println!("{}", serde_json::to_string(&next)?);
                    }
                }
            }
            "object" => {
                for object in object.object.iter_mut() {
                    let object = object.resolved().await?;
                    if args.debug {
                        println!("{:#?}", object);
                    } else {
                        println!("{}", serde_json::to_string(&object)?);
                    }
                }
            }
            "prev" => {
                for prev in object.prev.iter_mut() {
                    let prev = prev.resolved().await?;
                    if args.debug {
                        println!("{:#?}", prev);
                    } else {
                        println!("{}", serde_json::to_string(&prev)?);
                    }
                }
            }
            "preview" => {
                for preview in object.preview.iter_mut() {
                    let preview = preview.resolved().await?;
                    if args.debug {
                        println!("{:#?}", preview);
                    } else {
                        println!("{}", serde_json::to_string(&preview)?);
                    }
                }
            }
            "result" => {
                for result in object.result.iter_mut() {
                    let result = result.resolved().await?;
                    if args.debug {
                        println!("{:#?}", result);
                    } else {
                        println!("{}", serde_json::to_string(&result)?);
                    }
                }
            }
            "replies" => {
                for replies in object.replies.iter_mut() {
                    let replies = replies.resolved().await?;
                    if args.debug {
                        println!("{:#?}", replies);
                    } else {
                        println!("{}", serde_json::to_string(&replies)?);
                    }
                }
            }
            "tag" => {
                for tag in object.tag.iter_mut() {
                    let tag = tag.resolved().await?;
                    if args.debug {
                        println!("{:#?}", tag);
                    } else {
                        println!("{}", serde_json::to_string(&tag)?);
                    }
                }
            }
            "target" => {
                for target in object.target.iter_mut() {
                    let target = target.resolved().await?;
                    if args.debug {
                        println!("{:#?}", target);
                    } else {
                        println!("{}", serde_json::to_string(&target)?);
                    }
                }
            }
            "to" => {
                for to in object.to.iter_mut() {
                    let to = to.resolved().await?;
                    if args.debug {
                        println!("{:#?}", to);
                    } else {
                        println!("{}", serde_json::to_string(&to)?);
                    }
                }
            }
            "url" => {
                for url in object.url.iter_mut() {
                    let url = url.resolved().await?;
                    if args.debug {
                        println!("{:#?}", url);
                    } else {
                        println!("{}", serde_json::to_string(&url)?);
                    }
                }
            }
            "part_of" => {
                for part_of in object.part_of.iter_mut() {
                    let part_of = part_of.resolved().await?;
                    if args.debug {
                        println!("{:#?}", part_of);
                    } else {
                        println!("{}", serde_json::to_string(&part_of)?);
                    }
                }
            }
            "subject" => {
                for subject in object.subject.iter_mut() {
                    let subject = subject.resolved().await?;
                    if args.debug {
                        println!("{:#?}", subject);
                    } else {
                        println!("{}", serde_json::to_string(&subject)?);
                    }
                }
            }
            "relationship" => {
                for relationship in object.relationship.iter_mut() {
                    let relationship = relationship.resolved().await?;
                    if args.debug {
                        println!("{:#?}", relationship);
                    } else {
                        println!("{}", serde_json::to_string(&relationship)?);
                    }
                }
            }
            "former_type" => {
                for former_type in object.former_type.iter_mut() {
                    let former_type = former_type.resolved().await?;
                    if args.debug {
                        println!("{:#?}", former_type);
                    } else {
                        println!("{}", serde_json::to_string(&former_type)?);
                    }
                }
            }
            "inbox" => {
                for inbox in object.inbox.iter_mut() {
                    let inbox = inbox.resolved().await?;
                    if args.debug {
                        println!("{:#?}", inbox);
                    } else {
                        println!("{}", serde_json::to_string(&inbox)?);
                    }
                }
            }
            "outbox" => {
                for outbox in object.outbox.iter_mut() {
                    let outbox = outbox.resolved().await?;
                    if args.debug {
                        println!("{:#?}", outbox);
                    } else {
                        println!("{}", serde_json::to_string(&outbox)?);
                    }
                }
            }
            "following" => {
                for following in object.following.iter_mut() {
                    let following = following.resolved().await?;
                    if args.debug {
                        println!("{:#?}", following);
                    } else {
                        println!("{}", serde_json::to_string(&following)?);
                    }
                }
            }
            "followers" => {
                for followers in object.followers.iter_mut() {
                    let followers = followers.resolved().await?;
                    if args.debug {
                        println!("{:#?}", followers);
                    } else {
                        println!("{}", serde_json::to_string(&followers)?);
                    }
                }
            }
            "liked" => {
                for liked in object.liked.iter_mut() {
                    let liked = liked.resolved().await?;
                    if args.debug {
                        println!("{:#?}", liked);
                    } else {
                        println!("{}", serde_json::to_string(&liked)?);
                    }
                }
            }
            "streams" => {
                for streams in object.streams.iter_mut() {
                    let streams = streams.resolved().await?;
                    if args.debug {
                        println!("{:#?}", streams);
                    } else {
                        println!("{}", serde_json::to_string(&streams)?);
                    }
                }
            }

            "describes" => {
                if let Some(describes) = object.describes.as_ref() {
                    if args.debug {
                        println!("{:#?}", describes);
                    } else {
                        println!("{}", serde_json::to_string(describes)?);
                    }
                }
            }
            "source" => {
                if let Some(describes) = object.describes.as_ref() {
                    if args.debug {
                        println!("{:#?}", describes);
                    } else {
                        println!("{}", serde_json::to_string(describes)?);
                    }
                }
            }
            "rest" => {
                if args.debug {
                    println!("{:#?}", object.rest);
                } else {
                    println!("{}", serde_json::to_string(&object.rest)?);
                }
            }

            other => {
                #[derive(Debug)]
                struct NonResolvableProperty(String);
                impl std::fmt::Display for NonResolvableProperty {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "non resolvable property: {:?}", self.0)
                    }
                }
                impl std::error::Error for NonResolvableProperty {}

                return Err(Box::new(NonResolvableProperty(String::from(other)))
                    as Box<dyn std::error::Error>);
            }
        }
    } else {
        if args.debug {
            println!("{:#?}", object)
        } else {
            println!("{}", serde_json::to_string(&object)?);
        }
    };

    Ok(())
}
