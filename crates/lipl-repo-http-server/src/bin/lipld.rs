#[tokio::main(flavor = "current_thread") ]
async fn main() -> anyhow::Result<()> {
    lipl_repo_http_server::run().await
}
