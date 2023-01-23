#[tokio::main(flavor = "current_thread") ]
async fn main() -> lipl_core::Result<()> {
    lipl_server_warp::run().await
}
