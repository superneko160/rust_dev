use axum::{
    extract::{Form, Path},
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

#[derive(Deserialize, Serialize)]
struct FormInput {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let app = app();

    // TCPリスナのバインド
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    // サーバ起動
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    // ルーティング設定
    Router::new()
        .route("/", get(index))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user))
        .route("/contact", get(contact).post(accept_contact))
        .fallback(handler_404)
}

async fn index() -> &'static str {
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

async fn accept_contact(Form(input): Form<FormInput>) -> Html<String> {
    Html(format!("name: {} (email: {})", &input.name, &input.email))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_index_returns_200() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_user_returns_200() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_user_returns_200() {
        let app = app();

        let user_json = r#"{"id": 1, "name": "Bob"}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(user_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_contact_returns_200() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/contact")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_accept_contact_returns_200() {
        let app = app();

        let form_data = "name=John&email=john@example.com";

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/contact")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from(form_data))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_non_existent_url_returns_404() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/non-existent-path")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
