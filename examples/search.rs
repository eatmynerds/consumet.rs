use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.search("hi".to_owned(), Some(1)).await?
    );

    Ok(())
}
