#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut object = serde_json::from_reader::<_, smacktivity::Object>(std::io::stdin())?;

    if std::env::args().any(|arg| arg == "--resolve") {
        object = object.resolve().await?;
    }

    if std::env::args().any(|arg| arg == "--debug") {
        println!("{:#?}", object)
    } else {
        println!("{}", serde_json::to_string(&object)?);
    }

    Ok(())
}
