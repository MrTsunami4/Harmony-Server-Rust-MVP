use axum::{
    extract::{Json, Path},
    handler::get,
    Router,
};

use sled::Config;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let config = Config::new();

    let app = Router::new()
        .route("/", get(get_slash).post(post_name))
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

async fn post_name(Json(payload): Json<User>) -> String {
    format!(
        "Son nom est {} et son niveau est {}",
        payload.username, payload.level
    )
}

async fn get_name(Path((username, level)): Path<(String, u32)>) -> Json<User> {
    let user = User { username, level };
    Json(user)
}

async fn get_number(Path(number): Path<u32>) -> String {
    format!("Le nombre plus un est : {}", number + 1)
}

async fn get_timed(Path(number): Path<u64>) -> String {
    format!("Le nombre fois 2 est {}", number * 2)
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    level: u32,
}
