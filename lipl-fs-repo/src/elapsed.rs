use async_trait::{async_trait};
use futures::{Future};
use anyhow::{Result};

#[async_trait]
pub trait Elapsed
{
    async fn elapsed(&self) -> Result<u128>;
}

#[async_trait]
impl<T, F, Fut> Elapsed for F
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output=Result<T>> + Send,
{
    async fn elapsed(&self) -> Result<u128> {
        let now = std::time::Instant::now();
        self().await?;
        Ok(now.elapsed().as_millis())
    }
}


#[cfg(test)]
mod test {

    // #[tokio::test]
    // async fn elapsed() {
    //     use std::time::{Duration};
    //     use anyhow::Error;
    //     use tokio::time::sleep;

    //     use super::Elapsed;

    //     let millis: u128 = 2;
    //     let timeout = Duration::from_millis(millis as u64);
    //     let process = || async move {
    //         sleep(timeout).await;
    //         Ok::<(), Error>(())
    //     };
    //     assert_eq!(process.elapsed().await.ok(), Some(millis + 1));
    // }
}