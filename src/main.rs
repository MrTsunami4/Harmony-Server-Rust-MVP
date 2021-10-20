use axum::{extract::Path, handler::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_slash))
        .route("/cat", get(get_cat))
        .route("/user/:name/:level", get(get_name))
        .route("/plus/:number", get(get_number))
        .route("/time/:number", get(get_timed));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_slash() -> String {
    "Hello world".to_string()
}

async fn get_cat() -> String {
    "Meow".to_string()
}

async fn get_name(Path((username, level)): Path<(String, u32)>) -> Json<User> {
    let user = User { username, level };
    Json(user)
}

async fn get_number(Path(number): Path<u32>) -> String {
    format!("Le nombre plus un est : {}", number + 1).to_string()
}

async fn get_timed(Path(number): Path<u64>) -> String {
    format!("Le nombre fois 2 est {}", number * 2).to_string()
}

#[derive(Serialize)]
struct User {
    username: String,
    level: u32,
}
