use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
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

pub const ACTIVITYSTREAMS_CONTEXT: &str = "https://www.w3.org/ns/activitystreams";

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamsContext {
    Url(Url),
    String(String),
    Map(HashMap<String, ActivityStreamsContext>),
    List(Vec<ActivityStreamsContext>),
}

impl Default for ActivityStreamsContext {
    fn default() -> Self {
        ActivityStreamsContext::Url(Url::parse(ACTIVITYSTREAMS_CONTEXT).unwrap())
    }
}

impl std::fmt::Debug for ActivityStreamsContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityStreamsContext::Url(url) => {
                f.debug_tuple("Context").field(&format!("{}", url)).finish()
            }
            ActivityStreamsContext::String(string) => {
                f.debug_tuple("Context").field(&string).finish()
            }
            ActivityStreamsContext::Map(map) => f.debug_tuple("Context").field(&map).finish(),
            ActivityStreamsContext::List(list) => f.debug_tuple("Context").field(&list).finish(),
        }
    }
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum NonFunctional<T> {
    #[default]
    None,
    One(T),
    Many(Vec<T>),
}

impl<T> NonFunctional<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, NonFunctional::None)
    }

    pub fn iter(&self) -> NonFunctionalIter<'_, T> {
        NonFunctionalIter {
            non_functional: self,
            index: 0,
        }
    }

    pub fn iter_mut<'this>(&'this mut self) -> NonFunctionalIterMut<'this, T> {
        NonFunctionalIterMut {
            non_functional: self,
            index: 0,
        }
    }
}

pub struct NonFunctionalIter<'nf, T> {
    non_functional: &'nf NonFunctional<T>,
    index: usize,
}

impl<'nf, T> Iterator for NonFunctionalIter<'nf, T> {
    type Item = &'nf T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.non_functional {
            NonFunctional::One(one) if self.index == 0 => one,
            NonFunctional::Many(many) => many.get(self.index)?,
            _ => return None,
        };
        self.index += 1;
        Some(next)
    }
}

pub struct NonFunctionalIterMut<'nf, T> {
    non_functional: &'nf mut NonFunctional<T>,
    index: usize,
}

impl<'nf, T> Iterator for NonFunctionalIterMut<'nf, T> {
    type Item = &'nf mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.non_functional {
            NonFunctional::One(one) if self.index == 0 => one as *mut T,
            NonFunctional::Many(many) => many.get_mut(self.index)? as *mut T,
            _ => return None,
        };
        self.index += 1;

        // SAFETY:
        // - `next` points to a valid T
        // - The lifetime 'nf is equal to 'this from NonFunctional::iter_mut
        // - The lifetime of &mut self is irrelevant, since we are borrowing
        //   from `self.non_functional` with lifetime 'nf rather than self
        // - It is not possible to obtain multiple NonFunctionalIterMuts from a
        //   single NonFunctional
        Some(unsafe { &mut *next })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ClosedProperty {
    String(String), // TODO datetime
    Bool(bool),
    Object(LinkObject),
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EndpointsProperty {
    proxy_url: Option<Url>,
    oauth_authorization_endpoint: Option<Url>,
    oauth_token_endpoint: Option<Url>,
    provide_client_key: Option<Url>,
    sign_client_key: Option<Url>,
    shared_inbox: Option<Url>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LinkObject {
    Url(Url),
    Object(Box<Object>),
}

impl LinkObject {
    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            LinkObject::Object(object) => Some(object.as_mut()),
            _ => None,
        }
    }
}

impl std::fmt::Debug for LinkObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkObject::Url(url) => f.debug_tuple("Link").field(&format!("{}", url)).finish(),
            LinkObject::Object(object) => object.fmt(f),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum LinkRelation {
    Alternate,
    Canonical,
    Author,
    Bookmark,
    DnsPrefetch,
    External,
    Help,
    Icon,
    Manifest,
    Modulepreload,
    License,
    Next,
    Nofollow,
    Noopener,
    Noreferrer,
    Opener,
    Pingback,
    Preconnect,
    Prefetch,
    Preload,
    Prev,
    Search,
    Stylesheet,
    Tag,
}

#[derive(Debug)]
pub enum Units {
    Cm,
    Feet,
    Inches,
    Km,
    M,
    Miles,
    Url(Url),
}

impl serde::Serialize for Units {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Units::Cm => serializer.serialize_str("cm"),
            Units::Feet => serializer.serialize_str("feet"),
            Units::Inches => serializer.serialize_str("inches"),
            Units::Km => serializer.serialize_str("km"),
            Units::M => serializer.serialize_str("m"),
            Units::Miles => serializer.serialize_str("miles"),
            Units::Url(url) => url.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Units {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UnitsVisitor;
        impl<'de> serde::de::Visitor<'de> for UnitsVisitor {
            type Value = Units;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Unit or URL")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Ok(url) = Url::parse(v) {
                    return Ok(Units::Url(url));
                }

                match v {
                    "cm" => Ok(Units::Cm),
                    "feet" => Ok(Units::Feet),
                    "inches" => Ok(Units::Inches),
                    "km" => Ok(Units::Km),
                    "m" => Ok(Units::M),
                    "miles" => Ok(Units::Miles),
                    _ => Err(serde::de::Error::custom(format!("unknown unit {}", v))),
                }
            }
        }

        deserializer.deserialize_str(UnitsVisitor)
    }
}

#[rustfmt::skip]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "@context")]
    pub schema_context: ActivityStreamsContext,

    #[serde(rename = "type")]
    pub type_: ActivityStreamsType,

    #[serde(skip_serializing_if = "Option::is_none")]        pub id: Option<Url>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub actor: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub attachment: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub attributed_to: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub audience: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub bcc: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub bto: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub cc: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub context: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub current: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub first: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub generator: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub icon: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub image: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub in_reply_to: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub instrument: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub last: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub location: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub items: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub ordered_items: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub one_of: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub any_of: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub closed: Option<ClosedProperty>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub origin: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub next: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub object: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub prev: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub preview: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub result: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub replies: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub tag: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub target: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub to: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub url: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub altitude: Option<f32>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub content: NonFunctional<String>, // TODO - langString/contentMap?
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub name: NonFunctional<String>, // TODO - langString/nameMap?
    #[serde(skip_serializing_if = "Option::is_none")]        pub duration: Option<String>, // TODO - duration
    #[serde(skip_serializing_if = "Option::is_none")]        pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub href: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub hreflang: Option<String>, // TODO - language tag
    #[serde(skip_serializing_if = "Option::is_none")]        pub part_of: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub latitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub longitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub end_time: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]        pub published: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]        pub start_time: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]        pub radius: Option<f32>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub rel: NonFunctional<LinkRelation>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub start_index: Option<u32>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub summary: NonFunctional<String>, // TODO - langString/summaryMap?
    #[serde(skip_serializing_if = "Option::is_none")]        pub total_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub units: Option<Units>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub updated: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]        pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub subject: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub relationship: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub describes: Option<Box<Object>>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub former_type: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub deleted: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]        pub source: Option<Box<Object>>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub inbox: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub outbox: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub following: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub followers: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub liked: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")] pub streams: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub endpoints: Option<EndpointsProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]        pub preferred_username: Option<String>,

    #[serde(flatten)]
    pub rest: HashMap<String, serde_json::Value>,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            schema_context: ActivityStreamsContext::default(),
            type_: ActivityStreamsType::Object,
            id: None,
            actor: NonFunctional::None,
            attachment: NonFunctional::None,
            attributed_to: NonFunctional::None,
            audience: NonFunctional::None,
            bcc: NonFunctional::None,
            bto: NonFunctional::None,
            cc: NonFunctional::None,
            context: NonFunctional::None,
            current: None,
            first: None,
            generator: NonFunctional::None,
            icon: NonFunctional::None,
            image: NonFunctional::None,
            in_reply_to: NonFunctional::None,
            instrument: NonFunctional::None,
            last: None,
            location: NonFunctional::None,
            items: NonFunctional::None,
            ordered_items: NonFunctional::None,
            one_of: NonFunctional::None,
            any_of: NonFunctional::None,
            closed: None,
            origin: NonFunctional::None,
            next: None,
            object: NonFunctional::None,
            prev: None,
            preview: NonFunctional::None,
            result: NonFunctional::None,
            replies: NonFunctional::None,
            tag: NonFunctional::None,
            target: NonFunctional::None,
            to: NonFunctional::None,
            url: NonFunctional::None,
            accuracy: None,
            altitude: None,
            content: NonFunctional::None,
            name: NonFunctional::None,
            duration: None,
            height: None,
            href: None,
            hreflang: None,
            part_of: None,
            latitude: None,
            longitude: None,
            media_type: None,
            end_time: None,
            published: None,
            start_time: None,
            radius: None,
            rel: NonFunctional::None,
            start_index: None,
            summary: NonFunctional::None,
            total_items: None,
            units: None,
            updated: None,
            width: None,
            subject: None,
            relationship: NonFunctional::None,
            describes: None,
            former_type: NonFunctional::None,
            deleted: None,

            source: None,
            inbox: None,
            outbox: None,
            following: None,
            followers: None,
            liked: None,
            streams: NonFunctional::None,
            endpoints: None,
            preferred_username: None,

            rest: HashMap::default(),
        }
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Object");

        dbg.field("@context", &self.schema_context);
        dbg.field("type", &self.type_);
        if let Some(id) = self.id.as_ref() {
            dbg.field("id", &format!("{}", id));
        }
        match &self.actor {
            NonFunctional::One(one) => {
                dbg.field("actor", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("actor", many);
            }
            NonFunctional::None => {}
        }
        match &self.attachment {
            NonFunctional::One(one) => {
                dbg.field("attachment", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("attachment", many);
            }
            NonFunctional::None => {}
        }
        match &self.attributed_to {
            NonFunctional::One(one) => {
                dbg.field("attributed_to", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("attributed_to", many);
            }
            NonFunctional::None => {}
        }
        match &self.audience {
            NonFunctional::One(one) => {
                dbg.field("audience", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("audience", many);
            }
            NonFunctional::None => {}
        }
        match &self.bcc {
            NonFunctional::One(one) => {
                dbg.field("bcc", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("bcc", many);
            }
            NonFunctional::None => {}
        }
        match &self.bto {
            NonFunctional::One(one) => {
                dbg.field("bto", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("bto", many);
            }
            NonFunctional::None => {}
        }
        match &self.cc {
            NonFunctional::One(one) => {
                dbg.field("cc", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("cc", many);
            }
            NonFunctional::None => {}
        }
        match &self.context {
            NonFunctional::One(one) => {
                dbg.field("context", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("context", many);
            }
            NonFunctional::None => {}
        }
        if let Some(current) = self.current.as_ref() {
            dbg.field("current", &current);
        }
        if let Some(first) = self.first.as_ref() {
            dbg.field("first", &first);
        }
        match &self.generator {
            NonFunctional::One(one) => {
                dbg.field("generator", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("generator", many);
            }
            NonFunctional::None => {}
        }
        match &self.icon {
            NonFunctional::One(one) => {
                dbg.field("icon", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("icon", many);
            }
            NonFunctional::None => {}
        }
        match &self.image {
            NonFunctional::One(one) => {
                dbg.field("image", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("image", many);
            }
            NonFunctional::None => {}
        }
        match &self.in_reply_to {
            NonFunctional::One(one) => {
                dbg.field("in_reply_to", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("in_reply_to", many);
            }
            NonFunctional::None => {}
        }
        match &self.instrument {
            NonFunctional::One(one) => {
                dbg.field("instrument", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("instrument", many);
            }
            NonFunctional::None => {}
        }
        if let Some(last) = self.last.as_ref() {
            dbg.field("last", &last);
        }
        match &self.location {
            NonFunctional::One(one) => {
                dbg.field("location", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("location", many);
            }
            NonFunctional::None => {}
        }
        match &self.items {
            NonFunctional::One(one) => {
                dbg.field("items", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("items", many);
            }
            NonFunctional::None => {}
        }
        match &self.ordered_items {
            NonFunctional::One(one) => {
                dbg.field("ordered_items", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("ordered_items", many);
            }
            NonFunctional::None => {}
        }
        match &self.one_of {
            NonFunctional::One(one) => {
                dbg.field("one_of", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("one_of", many);
            }
            NonFunctional::None => {}
        }
        match &self.any_of {
            NonFunctional::One(one) => {
                dbg.field("any_of", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("any_of", many);
            }
            NonFunctional::None => {}
        }
        if let Some(closed) = self.closed.as_ref() {
            dbg.field("closed", &closed);
        }
        match &self.origin {
            NonFunctional::One(one) => {
                dbg.field("origin", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("origin", many);
            }
            NonFunctional::None => {}
        }
        if let Some(next) = self.next.as_ref() {
            dbg.field("next", &next);
        }
        match &self.object {
            NonFunctional::One(one) => {
                dbg.field("object", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("object", many);
            }
            NonFunctional::None => {}
        }
        if let Some(prev) = self.prev.as_ref() {
            dbg.field("prev", &prev);
        }
        match &self.preview {
            NonFunctional::One(one) => {
                dbg.field("preview", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("preview", many);
            }
            NonFunctional::None => {}
        }
        match &self.result {
            NonFunctional::One(one) => {
                dbg.field("result", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("result", many);
            }
            NonFunctional::None => {}
        }
        match &self.replies {
            NonFunctional::One(one) => {
                dbg.field("replies", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("replies", many);
            }
            NonFunctional::None => {}
        }
        match &self.tag {
            NonFunctional::One(one) => {
                dbg.field("tag", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("tag", many);
            }
            NonFunctional::None => {}
        }
        match &self.target {
            NonFunctional::One(one) => {
                dbg.field("target", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("target", many);
            }
            NonFunctional::None => {}
        }
        match &self.to {
            NonFunctional::One(one) => {
                dbg.field("to", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("to", many);
            }
            NonFunctional::None => {}
        }
        match &self.url {
            NonFunctional::One(one) => {
                dbg.field("url", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("url", many);
            }
            NonFunctional::None => {}
        }
        if let Some(accuracy) = self.accuracy.as_ref() {
            dbg.field("accuracy", &accuracy);
        }
        if let Some(altitude) = self.altitude.as_ref() {
            dbg.field("altitude", &altitude);
        }
        match &self.content {
            NonFunctional::One(one) => {
                dbg.field("content", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("content", many);
            }
            NonFunctional::None => {}
        }
        match &self.name {
            NonFunctional::One(one) => {
                dbg.field("name", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("name", many);
            }
            NonFunctional::None => {}
        }
        if let Some(duration) = self.duration.as_ref() {
            dbg.field("duration", &duration);
        }
        if let Some(height) = self.height.as_ref() {
            dbg.field("height", &height);
        }
        if let Some(href) = self.href.as_ref() {
            dbg.field("href", &format!("{}", href));
        }
        if let Some(hreflang) = self.hreflang.as_ref() {
            dbg.field("hreflang", &hreflang);
        }
        if let Some(part_of) = self.part_of.as_ref() {
            dbg.field("part_of", &part_of);
        }
        if let Some(latitude) = self.latitude.as_ref() {
            dbg.field("latitude", &latitude);
        }
        if let Some(longitude) = self.longitude.as_ref() {
            dbg.field("longitude", &longitude);
        }
        if let Some(media_type) = self.media_type.as_ref() {
            dbg.field("media_type", &media_type);
        }
        if let Some(end_time) = self.end_time.as_ref() {
            dbg.field("end_time", &end_time);
        }
        if let Some(published) = self.published.as_ref() {
            dbg.field("published", &published);
        }
        if let Some(start_time) = self.start_time.as_ref() {
            dbg.field("start_time", &start_time);
        }
        if let Some(radius) = self.radius.as_ref() {
            dbg.field("radius", &radius);
        }
        match &self.rel {
            NonFunctional::One(one) => {
                dbg.field("rel", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("rel", many);
            }
            NonFunctional::None => {}
        }
        if let Some(start_index) = self.start_index.as_ref() {
            dbg.field("start_index", &start_index);
        }
        match &self.summary {
            NonFunctional::One(one) => {
                dbg.field("summary", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("summary", many);
            }
            NonFunctional::None => {}
        }
        if let Some(total_items) = self.total_items.as_ref() {
            dbg.field("total_items", &total_items);
        }
        if let Some(units) = self.units.as_ref() {
            dbg.field("units", &units);
        }
        if let Some(updated) = self.updated.as_ref() {
            dbg.field("updated", &updated);
        }
        if let Some(width) = self.width.as_ref() {
            dbg.field("width", &width);
        }
        if let Some(subject) = self.subject.as_ref() {
            dbg.field("subject", &subject);
        }
        match &self.relationship {
            NonFunctional::One(one) => {
                dbg.field("relationship", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("relationship", many);
            }
            NonFunctional::None => {}
        }
        if let Some(describes) = self.describes.as_ref() {
            dbg.field("describes", &describes);
        }
        match &self.former_type {
            NonFunctional::One(one) => {
                dbg.field("former_type", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("former_type", many);
            }
            NonFunctional::None => {}
        }
        if let Some(deleted) = self.deleted.as_ref() {
            dbg.field("deleted", &deleted);
        }
        if let Some(source) = self.source.as_ref() {
            dbg.field("source", &source);
        }
        if let Some(inbox) = self.inbox.as_ref() {
            dbg.field("inbox", &inbox);
        }
        if let Some(outbox) = self.outbox.as_ref() {
            dbg.field("outbox", &outbox);
        }
        if let Some(following) = self.following.as_ref() {
            dbg.field("following", &following);
        }
        if let Some(followers) = self.followers.as_ref() {
            dbg.field("followers", &followers);
        }
        if let Some(liked) = self.liked.as_ref() {
            dbg.field("liked", &liked);
        }
        match &self.streams {
            NonFunctional::One(one) => {
                dbg.field("streams", &one);
            }
            NonFunctional::Many(many) => {
                dbg.field("streams", &many);
            }
            _ => {}
        }
        if let Some(endpoints) = self.endpoints.as_ref() {
            dbg.field("endpoints", &endpoints);
        }
        if let Some(preferred_username) = self.preferred_username.as_ref() {
            dbg.field("preferred_username", &preferred_username);
        }

        if !self.rest.is_empty() {
            dbg.field("(rest)", &self.rest);
        }

        dbg.finish()
    }
}
