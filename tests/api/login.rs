use crate::helpers::{assert_is_redirect_to, spawn_app};
use reqwest::header::HeaderValue;
use std::collections::HashSet;
#[actix_web::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    // Try logging in
    let response = app.post_login(&login_body).await;
    // See if page redirected
    assert_is_redirect_to(&response, "/login");
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>Authentication failed</i></p>"));

    // Reload login page
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains("<p><i>Authentication failed.</i></p>"));
}