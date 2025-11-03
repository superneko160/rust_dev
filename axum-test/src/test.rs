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
async fn test_accept_contact_with_empty_name_returns_400() {
    // 名前が空文字列の場合、400エラーを返す
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
async fn test_accept_contact_with_invalid_email_returns_400() {
    // メールアドレスの形式が不正な場合、400エラーを返す
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
async fn test_accept_contact_with_empty_email_returns_400() {
    // メールアドレスが空文字列の場合、400エラーを返す
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
async fn test_accept_contact_with_both_empty_returns_400() {
    // 名前とメールアドレスの両方が空文字列の場合、400エラーを返す
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
