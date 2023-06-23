use url::Url;

fn main() {
    let obj = smacktivity::Object {
        id: Some(smacktivity::IdProperty(
            Url::parse("https://grape.surgery/smack/outbox").unwrap(),
        )),
        type_: smacktivity::TypeProperty(smacktivity::ActivityStreamsType::Actor),
        deleted: Some(smacktivity::DeletedProperty(String::from("eysterday lol"))),
        ..Default::default()
    };

    //println!("{:#?}", obj);

    println!("{}", serde_json::to_string(&obj).unwrap());

    //let obj2 = smacktivity::object2!();
    //    let object: smacktivity::Object = serde_json::from_str(&format!(
    //        "{{
    //    \"@context\": \"{}\",
    //    \"id\": \"https://grape.surgery/smack/outbox\",
    //    \"type\": \"Actor\"
    //}}",
    //        smacktivity::ACTIVITYSTREAMS_CONTEXT,
    //    ))
    //    .unwrap();
    //
    //    println!("{}", serde_json::to_string(&object).unwrap());
}
