use criterion::{Criterion, criterion_group, criterion_main};
use lipl_core::Uuid;
use turso::{Rows, params};

async fn create_db() -> turso::Connection {
    let db = turso::Builder::new_local(":memory:").build().await.unwrap();
    let con = db.connect().unwrap();
    con.execute_batch(include_str!("../src/create_db.sql"))
        .await
        .unwrap();
    con
}

async fn insert_lyric(con: &turso::Connection, title: &str, parts: &str) {
    con.execute(
        "INSERT INTO lyric (id, title, parts) VALUES ($1, $2, $3)",
        params!(Uuid::default().to_string().as_str(), title, parts),
    )
    .await
    .unwrap();
}

async fn list_lyrics(con: &turso::Connection) -> Rows {
    con.query(
        "SELECT id, title, parts FROM lyric ORDER BY title",
        params!(),
    )
    .await
    .unwrap()
}

fn memory_db() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let con = create_db().await;
            insert_lyric(&con, "Er is er één jarig", "").await;
            list_lyrics(&con).await;
        });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("memory_db", |b| b.iter(|| memory_db()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
