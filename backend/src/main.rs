use axum::{
    body::Body,
    handler::Handler,
    http::{Request, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Serve static files from the `dist` folder
    let serve_dir = ServeDir::new("frontend/dist");

    // Set up Axum router
    let app = Router::new()
        .nest_service("/", serve_dir) // Serve the static frontend
        .fallback(fallback.into_service()); // Handle 404s

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Fallback handler for unmatched routes
async fn fallback(req: Request<Body>) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route found for {}", req.uri()))
}
