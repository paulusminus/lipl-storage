#[tokio::main(flavor = "current_thread") ]
async fn main() -> lipl_core::Result<()> {
    lipl_repo_http_server::run().await
}
