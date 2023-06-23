fn main() -> Result<(), serde_json::Error> {
    let object = serde_json::from_reader::<_, smacktivity::Object>(std::io::stdin())?;
    println!("{:#?}", object);
    Ok(())
}
