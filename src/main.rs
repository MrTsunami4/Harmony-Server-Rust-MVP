use axum::{extract::Path, handler::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_slash))
        .route("/cat", get(get_cat))
        .route("/user/:name", get(get_name));

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

async fn get_name(Path(body): Path<String>) -> String {
    format!("Hello {}", body).to_string()
}
