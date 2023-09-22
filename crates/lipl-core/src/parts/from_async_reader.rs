use futures::{
    future::{ready, Ready},
    io::BufReader,
    AsyncRead, AsyncBufReadExt, TryStreamExt, TryFutureExt,
};
use lipl_util::VecExt;
use std::io::Error;

trait IntoReadyOk {
    fn into_ready_ok(self) -> Ready<Result<Self, Error>> where Self: Sized;
}

impl<T> IntoReadyOk for T {
    fn into_ready_ok(self) -> Ready<Result<T, Error>> {
        ready(Ok(self))
    }    
}

pub async fn from_async_reader<R>(r: R) -> Result<Vec<Vec<String>>, Error>
where
    R: AsyncRead,
{
    BufReader::new(r)
        .lines()
        .try_fold((true, Vec::<Vec<String>>::new()), |mut acc, next| {
            let line = next.trim().to_owned();
            if line.is_empty() {
                (true, acc.1)
                .into_ready_ok()
            }
            else if acc.0 {
                (false, acc.1.add_one(vec![line]))
                .into_ready_ok()
            }
            else {
                let last = acc.1.pop().unwrap_or_default().add_one(line);
                (false, acc.1.add_one(last))
                .into_ready_ok()
            }
        }
    )
    .map_ok(|r| r.1)
    .await
}

#[cfg(test)]
mod test {
    use super::from_async_reader;

    #[tokio::test]
    async fn test_from_async_reader() {
        let test = "\r  Hallo allema  \t";
        assert_eq!(
            from_async_reader(test.as_bytes()).await.unwrap(),
            vec![vec!["Hallo allema".to_owned()]]
        );

    }
}