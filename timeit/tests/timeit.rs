use timeit::timeit;

#[timeit(level = "trace")]
async fn get_str_async() -> &'static str {
    futures::future::ready("Cargo.toml").await
}

#[timeit(level = "debug")]
fn get_str() -> &'static str {
    "Cargo.toml"
}

#[test]
fn testen_maar() {
    tracing_subscriber::fmt()
    .with_test_writer()
    .with_max_level(tracing::Level::TRACE)
    .init();

    let result = futures::executor::block_on(get_str_async());
    assert_eq!(result, "Cargo.toml");

    assert_eq!(get_str(), "Cargo.toml")
}