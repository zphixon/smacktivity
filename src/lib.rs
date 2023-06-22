use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidContext,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidContext => f.debug_struct("InvalidContext").finish(),
        }
    }
}

pub const ACTIVITYSTREAMS_CONTEXT: &str = "https://www.w3.org/ns/activitystreams";

#[derive(Debug, Clone, Copy)]
pub enum ActivityStreamsType {
    Object,
    Link,

    Collection,
    OrderedCollection,
    CollectionPage,
    OrderedCollectionPage,

    Activity,
    Add,
    Announce,
    Undo,
    Update,
    View,
    Block,
    Create,
    Delete,
    Dislike,
    Flag,
    Follow,
    Ignore,
    Join,
    Leave,
    Like,
    Listen,
    Move,
    Read,
    Remove,
    Offer,
    Invite,
    Reject,
    TentativeReject,
    Accept,
    TentativeAccept,
    Arrive,
    IntransitiveActivity,
    Travel,
    Question,

    Actor,
    Application,
    Group,
    Organization,
    Person,
    Service,

    // extended object types
    Article,
    Audio,
    Document,
    Event,
    Image,
    Note,
    Page,
    Place,
    Profile,
    Relationship,
    Tombstone,
    Video,

    // extended link types
    Mention,
}

#[derive(Debug)]
pub enum ActivityStreamsContext {
    PlainString,
    List,
}

smacktivity_macros::object!(
    schema_context ("@context"): ActivityStreamsContext
        = SchemaContextProperty(ActivityStreamsContext::PlainString),
    type_ ("type"): ActivityStreamsType
        = TypeProperty(ActivityStreamsType::Object),
    id?: url::Url,
    actor?: Object,
    attachment?: Object,
    attributed_to?: Object,
    audience?: Object,
    bcc?: Object,
    bto?: Object,
    cc?: Object,
    context?: Object,
    current?: Object,
    first?: Object,
    generator?: Object,
    icon?: Object,
    image?: Object,
    in_reply_to?: Object,
    instrument?: Object,
    last?: Object,
    location?: Object,
    items?: Object,
    one_of?: Object,
    any_of?: Object,
    closed?: Object | String | bool, // TODO datetime
    origin?: Object,
    next?: Object,
    object?: Object,
    prev?: Object,
    preview?: Object,
    result?: Object,
    replies?: Object,
    tag?: Object,
    target?: Object,
    to?: Object,
    url?: Object | url::Url,
    accuracy?: f32,
    altitude?: f32,
    content?: String, // TODO - langString
    name?: String, // TODO - langString
    duration?: String, // TODO - duration
    height?: u32,
    href?: url::Url,
    hreflang?: String, // TODO - language tag
    part_of?: Object,
    latitude?: f32,
    longitude?: f32,
    media_type?: Object,
    end_time?: String, // TODO - dateTime
    published?: String, // TODO - dateTime
    start_time?: String, // TODO - dateTime
    radius?: f32,
    rel?: Object, // TODO - RFC5988/HTML5 Link Relation?
    start_index?: u32,
    summary?: String, // TODO - langString
    total_items?: u32,
    units?: String, // TODO - string enum
    updated?: String, // TODO - dateTime
    width?: u32,
    subject?: Object,
    relationship?: Object,
    describes?: Object,
    former_type?: Object,
    deleted?: String, // TODO - dateTime
);

impl<'de> serde::Deserialize<'de> for Object {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
