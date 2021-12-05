use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    AddExtensionLayer, Router,
};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sled::Db;

#[tokio::main]
async fn main() {
    let tree = sled::open("/tmp/sled_harmony").expect("Can't open database ");
    let tree = Arc::new(tree);

    let app = Router::new()
        .route("/", get(get_slash).post(post_name))
        .route("/cat", get(get_cat))
        .route("/user/:name/:level", get(get_name))
        .route("/plus/:number", get(get_number))
        .route("/time/:number", get(get_timed))
        .route("/db_user/:name/:level", get(get_user))
        .layer(AddExtensionLayer::new(tree.clone()))
        .route("/put_user/:name/:level", post(put_user))
        .layer(AddExtensionLayer::new(tree.clone()));

    axum::Server::bind(&str::parse("0.0.0.0:3000").unwrap())
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

async fn get_user(Path(username): Path<String>, Extension(tree): Extension<Arc<Db>>) -> String {
    let user = tree.get(username).unwrap();
    let user = user.expect("The user doesn't exist.");
    user.first().unwrap().to_string()
}

async fn put_user(
    Path((username, level)): Path<(String, u32)>,
    Extension(tree): Extension<Arc<Db>>,
) {
    tree.insert(&[username], &[level]);
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    level: u32,
}
