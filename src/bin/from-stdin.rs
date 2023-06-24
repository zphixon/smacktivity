use heck::ToSnakeCase;
use smacktivity::network::Resolve;

#[derive(argh::FromArgs)]
#[argh(description = "read an activitypub object from stdin and do something with it")]
struct Args {
    #[argh(option, description = "resolve a property")]
    resolve: Option<String>,

    #[argh(switch, description = "use debug printing rather than JSON")]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args: Args = argh::from_env();

    let mut object = serde_json::from_reader::<_, smacktivity::Object>(std::io::stdin())?;

    //smacktivity::resolve_all!(object.actor)?;
    //smacktivity::resolve_maybe!(object.first)?;

    if let Some(property) = args.resolve.as_ref() {
        match property.as_str().to_snake_case().as_str() {
            "actor" => {
                object.actor.resolve().await?;
            }
            "attachment" => {
                object.attachment.resolve().await?;
            }
            "attributed_to" => {
                object.attributed_to.resolve().await?;
            }
            "audience" => {
                object.audience.resolve().await?;
            }
            "bcc" => {
                object.bcc.resolve().await?;
            }
            "bto" => {
                object.bto.resolve().await?;
            }
            "cc" => {
                object.cc.resolve().await?;
            }
            "context" => {
                object.context.resolve().await?;
            }
            "current" => {
                object.current.resolve().await?;
            }
            "first" => {
                object.first.resolve().await?;
            }
            "generator" => {
                object.generator.resolve().await?;
            }
            "icon" => {
                object.icon.resolve().await?;
            }
            "image" => {
                object.image.resolve().await?;
            }
            "in_reply_to" => {
                object.in_reply_to.resolve().await?;
            }
            "instrument" => {
                object.instrument.resolve().await?;
            }
            "last" => {
                object.last.resolve().await?;
            }
            "location" => {
                object.location.resolve().await?;
            }
            "items" => {
                object.items.resolve().await?;
            }
            "ordered_items" => {
                object.ordered_items.resolve().await?;
            }
            "one_of" => {
                object.one_of.resolve().await?;
            }
            "any_of" => {
                object.any_of.resolve().await?;
            }
            "origin" => {
                object.origin.resolve().await?;
            }
            "next" => {
                object.next.resolve().await?;
            }
            "object" => {
                object.object.resolve().await?;
            }
            "prev" => {
                object.prev.resolve().await?;
            }
            "preview" => {
                object.preview.resolve().await?;
            }
            "result" => {
                object.result.resolve().await?;
            }
            "replies" => {
                object.replies.resolve().await?;
            }
            "tag" => {
                object.tag.resolve().await?;
            }
            "target" => {
                object.target.resolve().await?;
            }
            "to" => {
                object.to.resolve().await?;
            }
            "url" => {
                object.url.resolve().await?;
            }
            "part_of" => {
                object.part_of.resolve().await?;
            }
            "subject" => {
                object.subject.resolve().await?;
            }
            "relationship" => {
                object.relationship.resolve().await?;
            }
            "former_type" => {
                object.former_type.resolve().await?;
            }
            "inbox" => {
                object.inbox.resolve().await?;
            }
            "outbox" => {
                object.outbox.resolve().await?;
            }
            "following" => {
                object.following.resolve().await?;
            }
            "followers" => {
                object.followers.resolve().await?;
            }
            "liked" => {
                object.liked.resolve().await?;
            }
            "streams" => {
                object.streams.resolve().await?;
            }

            _ => {}
        }
    }

    if args.debug {
        println!("{:#?}", object)
    } else {
        println!("{}", serde_json::to_string(&object)?);
    }

    Ok(())
}
