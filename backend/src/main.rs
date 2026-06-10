use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Invalid PORT");

    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            log::info!("Health check requested");
            warp::reply::json(&serde_json::json!({
                "status": "healthy",
                "service": "blink-backend",
                "version": "0.1.0"
            }))
        });

    let api = warp::path("api")
        .and(health);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let routes = api.with(cors);

    println!("🚀 Blink Backend starting on port {}", port);
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}
