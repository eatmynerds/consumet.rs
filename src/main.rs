use consumet::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dramacool = movies::DramaCool;

    let data = dramacool.search("Vincenzo", None).await?;

    println!("{:?}", data);

    Ok(())
}
