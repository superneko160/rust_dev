use axum::{routing::{get, MethodRouter}, Router};
use std::net::SocketAddr;

/**
 * webフレームワークaxumのテスト
 */
#[tokio::main]
async fn main() {
    // ルーティング設定
    let app = Router::new()
        .merge(root())
        .merge(get_foo());
    // ポート開設
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/**
 * http://127.0.0.1:7878/
 */
fn root() -> Router {
    async fn handler() -> &'static str {
        "Hello, World!"
    }
    route("/", get(handler))
}

/**
 * http://127.0.0.1:7878/foo
 */
fn get_foo() -> Router {
    async fn handler() -> &'static str {
        "Hi from `GET /foo`"
    }
    route("/foo", get(handler))
}

/**
 * ルーティング
 * 最新バージョンでは、引数の部分をmethod_router: MethordRouter<()>と型指定する
 */
fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}