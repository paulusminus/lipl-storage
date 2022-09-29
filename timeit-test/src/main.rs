use timeit::timeit;
use tracing_subscriber::filter::LevelFilter;
use std::{error::Error, path::Path, fmt::Debug};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type IOResult<T> = std::result::Result<T, std::io::Error>;

const FILE: &str = ".gitignore";

#[timeit]
pub async fn read_file<P>(path: P) -> IOResult<String>
where P: AsRef<Path> + Debug
{
    tokio::fs::read_to_string(path).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(LevelFilter::TRACE).init();
    let result = read_file(FILE).await?;
    println!("File: {FILE}");
    println!("{result}");
    Ok(())
}
