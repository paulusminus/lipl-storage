use timeit::timeit;

#[timeit(level = "trace")]
async fn get_str() -> &'static str {
    futures::future::ready("Cargo.toml").await
}

#[test]
fn testen_maar() {
    tracing_subscriber::fmt()
    .with_test_writer()
    .with_max_level(tracing::Level::TRACE)
    .init();

    let result = futures::executor::block_on(get_str());
    assert_eq!(result, "Cargo.toml");
}