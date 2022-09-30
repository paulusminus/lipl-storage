use timeit::timeit;
use tracing_subscriber::filter::LevelFilter;
use std::{error::Error, path::{Path, PathBuf}, fmt::Debug};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type IOResult<T> = std::result::Result<T, std::io::Error>;
type VarResult<T> = std::result::Result<T, std::env::VarError>;

fn cargo_toml_file() -> VarResult<PathBuf> {
    std::env::var("CARGO_MANIFEST_DIR")
    .map(|dir| PathBuf::from(dir).join("Cargo.toml"))
}

#[timeit]
pub async fn read_file<P>(path: P) -> IOResult<String>
where P: AsRef<Path> + Debug
{
    tokio::fs::read_to_string(path).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(LevelFilter::TRACE).init();

    let filename = cargo_toml_file()?;
    let result = read_file(&filename).await?;
    println!("File: {}", filename.to_string_lossy());
    println!("{result}");
    Ok(())
}
