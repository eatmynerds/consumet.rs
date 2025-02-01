use consumet::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new instance of the FlixHQ provider
    let flixhq = movies::FlixHQ;

    // Search for a movie. In this case, "Vincenzo"
    let data = flixhq.search("Vincenzo", None).await?;
    println!("{:#?}", data);

    Ok(())
}
