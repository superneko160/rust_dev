use axum::{
    extract::{Form, Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

#[derive(Deserialize, Serialize)]
struct User {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
struct UserWithoutId {
    name: String,
}

#[derive(Deserialize, Serialize, Validate)]
struct FormInput {
    #[validate(length(min = 1, message = "名前を入力してください"))]
    name: String,
    #[validate(length(min = 1, message = "メールアドレスを入力してください"))]
    #[validate(email(message = "正しいメールアドレスの形式で入力してください"))]
    email: String,
}

#[tokio::main]
async fn main() {
    // TCPリスナのバインド
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    // サーバ起動
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    // ルーティング設定
    Router::new()
        .route("/", get(index))
        .route("/list", get(list))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user))
        .route("/contact", get(contact).post(accept_contact))
        .fallback(handler_404)
}

async fn index() -> &'static str {
    "Hello, World!!"
}

async fn list(Query(pagination): Query<Pagination>) -> Json<Pagination> {
    Json(pagination)
}

async fn get_user(Path(user_id): Path<Uuid>) -> Json<User> {
    let user = User {
        id: user_id,
        name: "Alice".to_string(),
    };

    Json(user)
}

async fn create_user(Json(params): Json<UserWithoutId>) -> Json<User> {
    let user = User {
        id: Uuid::new_v4(),
        name: params.name,
    };

    // ユーザ登録処理

    Json(user)
}

async fn contact() -> Html<String> {
    let contents = r#"
        <!doctype html>
        <html>
        <head>
            <title>Axum Form</title>
        </head>
        <body>
            <form action="" method="post">
                <label for="name">
                    Enter your name:
                    <input type="text" name="name">
                </label>
                <br>

                <label>
                    Enter your email:
                    <input type="email" name="email">
                </label>
                <br>

                <input type="submit" value="Subscribe!">
            </form>
        </body>
        </html>
    "#
    .to_string();

    Html(contents)
}

async fn accept_contact(Form(input): Form<FormInput>) -> impl IntoResponse {
    match input.validate() {
        Ok(_) => create_success_response(&input),
        Err(errors) => create_error_response(errors),
    }
}

/// バリデーション成功時のレスポンスを生成する
/// 入力された名前とメールアドレスを表示するHTMLを返す
fn create_success_response(input: &FormInput) -> Response {
    Html(format!("name: {} (email: {})", &input.name, &input.email)).into_response()
}

/// バリデーション失敗時のエラーレスポンスを生成する
/// エラーメッセージを一覧表示するHTMLと400ステータスコードを返す
fn create_error_response(errors: validator::ValidationErrors) -> Response {
    // バリデーションエラーからエラーメッセージの配列を抽出
    let error_messages = extract_error_messages(errors);

    let error_html = format!(
        r#"
        <!doctype html>
        <html>
        <head>
            <title>バリデーションエラー</title>
        </head>
        <body>
            <h1>エラーが発生しました</h1>
            <ul>
                {}
            </ul>
            <a href="/contact">戻る</a>
        </body>
        </html>
        "#,
        error_messages
            .iter()
            .map(|msg| format!("<li>{}</li>", msg))
            .collect::<Vec<_>>()
            .join("\n")
    );

    (StatusCode::BAD_REQUEST, Html(error_html)).into_response()
}

/// バリデーションエラーからエラーメッセージを抽出する
/// 各フィールドのエラー情報を走査し、メッセージを収集する
fn extract_error_messages(errors: validator::ValidationErrors) -> Vec<String> {
    errors
        .field_errors()
        .iter()
        .flat_map(|(field, errors)| {
            errors.iter().map(move |error| {
                // カスタムメッセージがあればそれを使用、なければデフォルトメッセージを利用
                error
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("{}のバリデーションエラー", field))
            })
        })
        .collect()
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

#[cfg(test)]
mod test;
