//! `cargo run -p admin` serves the panel on http://127.0.0.1:3002 (`PORT` overrides the
//! port, `ADMIN_DB` the database, by default `admin.sqlite` in the working directory).

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("ADMIN_DB").unwrap_or_else(|_| "sqlite:admin.sqlite".into());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3002u16);
    let db = admin::db::open(&url).await?;
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
    println!("http://127.0.0.1:{port}  (ada@example.com / analytical-engine)");
    axum::serve(listener, admin::router(db)).await?;
    Ok(())
}
