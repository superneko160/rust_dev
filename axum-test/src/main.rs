use axum::{
    response::Html,
    routing::get,
    Json,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u8,
    name: String,
}

#[tokio::main]
async fn main() {
    // ルーティング設定
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(get_user))
        .route("/axum", get(get_html));

    // TCPリスナのバインド
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    // サーバ起動
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!!"
}

async fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };

    Json(user)
}

async fn get_html() -> Html<String> {
    let contents = format!("
        <!doctype html>
        <html>
            <title>Axum</title>
        <body>
            <h1>Hello, Axum!!</h1>
        </body>
        </html>
    ");

    Html(contents)
}
