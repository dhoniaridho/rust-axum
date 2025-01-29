use app::routes::create_routes;
use axum::{routing::get, Router};
mod app;
mod adapters;
mod shared;
mod domains;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(create_routes());

    // run our app with hyper, listening globally on port 3000
    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => {
            println!("listening on http://localhost:3000");
            match axum::serve(l, app).await {
                Ok(()) => println!("Server closed"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
