use super::*;
use axum::{body::Body, http::Request};
use tower::util::ServiceExt;

#[tokio::test]
async fn test_index_returns_200() {
    let response = app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_list_returns_200() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/list?page=2&per_page=33")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_user_returns_200() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/95be61c6-ffdc-4283-8d3b-a5048a53cfbb")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user_returns_200() {
    let user_json = r#"{"name": "Alice"}"#;

    let response = app()
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
    let response = app()
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
    let form_data = "name=John&email=john@example.com";

    let response = app()
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
async fn test_accept_contact_returns_submitted_data() {
    let form_data = "name=Alice&email=test@example.com";

    let response = app()
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

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("Alice"));
    assert!(body_str.contains("test@example.com"));
}

#[tokio::test]
async fn test_accept_contact_with_empty_name_returns_400() {
    let form_data = "name=&email=john@example.com";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_accept_contact_with_empty_name_shows_error_message() {
    let form_data = "name=&email=john@example.com";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("名前を入力してください"));
}

#[tokio::test]
async fn test_accept_contact_with_empty_email_returns_400() {
    let form_data = "name=John&email=";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_accept_contact_with_invalid_email_returns_400() {
    let form_data = "name=John&email=invalid-email";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_accept_contact_with_email_without_at_returns_400() {
    // @マークのないメールアドレスの場合、400エラーを返す
    let form_data = "name=John&email=johnexample.com";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_accept_contact_with_invalid_email_shows_error_message() {
    let form_data = "name=John&email=invalid-email";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("正しいメールアドレスの形式で入力してください"));
}

#[tokio::test]
async fn test_accept_contact_with_both_empty_returns_400() {
    let form_data = "name=&email=";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_accept_contact_with_both_empty_shows_multiple_errors() {
    let form_data = "name=&email=";

    let response = app()
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("名前を入力してください"));
    assert!(body_str.contains("メールアドレスを入力してください"));
}

#[tokio::test]
async fn test_non_existent_url_returns_404() {
    let response = app()
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
