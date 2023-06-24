use std::{collections::HashMap, fmt::Display};

#[cfg(feature = "reqwest")]
pub mod network;

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ActivityStreamsContext {
    Url(url::Url),
    String(String),
    Map(HashMap<String, ActivityStreamsContext>),
    List(Vec<ActivityStreamsContext>),
}

impl Default for ActivityStreamsContext {
    fn default() -> Self {
        ActivityStreamsContext::Url(url::Url::parse(ACTIVITYSTREAMS_CONTEXT).unwrap())
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

    pub fn iter_mut(&mut self) -> NonFunctionalIterMut<'_, T> {
        NonFunctionalIterMut {
            non_functional: self,
            index: 0,
        }
    }
}

pub struct NonFunctionalIter<'a, T> {
    non_functional: &'a NonFunctional<T>,
    index: usize,
}

impl<'a, T> Iterator for NonFunctionalIter<'a, T> {
    type Item = &'a T;

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

pub struct NonFunctionalIterMut<'a, T> {
    non_functional: &'a mut NonFunctional<T>,
    index: usize,
}

impl<'a, T> Iterator for NonFunctionalIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.non_functional {
            NonFunctional::One(one) if self.index == 0 => one,
            NonFunctional::Many(many) => many.get_mut(self.index)?,
            _ => return None,
        };
        self.index += 1;
        // SAFETY: uuuhhhhhhhh,,,,
        Some(unsafe { std::mem::transmute::<&mut T, &mut T>(next) })
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
    proxy_url: Option<url::Url>,
    oauth_authorization_endpoint: Option<url::Url>,
    oauth_token_endpoint: Option<url::Url>,
    provide_client_key: Option<url::Url>,
    sign_client_key: Option<url::Url>,
    shared_inbox: Option<url::Url>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LinkObject {
    Url(url::Url),
    Object(Box<Object>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "@context")]
    pub schema_context: ActivityStreamsContext,
    #[serde(rename = "type")]
    pub type_: ActivityStreamsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<url::Url>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub actor: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub attachment: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub attributed_to: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub audience: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub bcc: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub bto: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub cc: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub context: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub generator: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub icon: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub image: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub in_reply_to: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub instrument: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub location: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub items: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub ordered_items: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub one_of: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub any_of: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<ClosedProperty>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub origin: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub object: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub preview: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub result: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub replies: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub tag: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub target: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub to: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub url: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f32>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub content: NonFunctional<String>, // TODO - langString/contentMap?
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub name: NonFunctional<String>, // TODO - langString/nameMap?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>, // TODO - duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<url::Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>, // TODO - language tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub rel: NonFunctional<String>, // TODO - RFC5988/HTML5 Link Relation?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u32>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub summary: NonFunctional<String>, // TODO - langString/summaryMap?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>, // TODO - string enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>, // TODO - dateTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<LinkObject>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub relationship: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub describes: Option<Box<Object>>,
    #[serde(skip_serializing_if = "NonFunctional::is_none")]
    pub former_type: NonFunctional<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>, // TODO - dateTime

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Object>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbox: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<LinkObject>,
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<LinkObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<EndpointsProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
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
            streams: Vec::with_capacity(0),
            endpoints: None,
            preferred_username: None,
        }
    }
}
