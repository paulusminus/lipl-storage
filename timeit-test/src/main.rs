use timeit::timeit;
use tracing::instrument;
use tracing_subscriber::filter::LevelFilter;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[instrument]
#[timeit]
pub async fn jaja(hallo: &str) -> Result<String> {
    Ok(hallo.into())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(LevelFilter::INFO).init();
    let result = jaja("hallo").await?;
    println!("Hello, {result}");
    Ok(())
}
