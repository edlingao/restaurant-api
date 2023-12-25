use crate::router::router;

mod router;
mod types;
mod consts;

#[tokio::main]
async fn main() {

    // Load .env file
    dotenv::dotenv().ok();

    let app = router();

    let addr = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(addr, app).await.unwrap();
}

