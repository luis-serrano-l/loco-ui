//! `cargo run`, then open http://127.0.0.1:3000.

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("http://127.0.0.1:3000");
    axum::serve(listener, {{crate}}::router()).await
}
