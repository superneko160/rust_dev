use axum::{
    extract::Path,
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    id: u8,
    name: String,
}

#[tokio::main]
async fn main() {
    // ルーティング設定
    let app = Router::new()
        .route("/", get(root))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user))
        .route("/axum", get(get_html));
    let app = app.fallback(handler_404);

    // TCPリスナのバインド
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    // サーバ起動
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!!"
}

async fn get_user(Path(user_id): Path<u8>) -> Json<User> {
    let user = User {
        id: user_id,
        name: "Alice".to_string(),
    };

    Json(user)
}

async fn create_user(Json(params): Json<User>) -> Json<User> {
    let user = User {
        id: params.id,
        name: params.name,
    };

    Json(user)
}

async fn get_html() -> Html<String> {
    let contents = format!(
        "
        <!doctype html>
        <html>
            <title>Axum</title>
        <body>
            <h1>Hello, Axum!!</h1>
        </body>
        </html>
    "
    );

    Html(contents)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
