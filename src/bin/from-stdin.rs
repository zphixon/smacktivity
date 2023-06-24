fn main() -> Result<(), serde_json::Error> {
    let object = serde_json::from_reader::<_, smacktivity::Object>(std::io::stdin())?;

    if matches!(std::env::args().last().as_deref(), Some("--debug")) {
        println!("{:#?}", object)
    } else {
        println!("{}", serde_json::to_string(&object)?);
    }

    Ok(())
}
