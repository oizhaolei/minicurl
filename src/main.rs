use clap::Parser;

#[derive(Parser)]
pub struct Config {
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    let body = reqwest::get(config.url).await?.text().await?;

    println!("{}", body);
    Ok(())
}
