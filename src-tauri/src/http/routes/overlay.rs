use axum::response::IntoResponse;
use reqwest::header::CONTENT_TYPE;

/// Embedded oauth response page for handling sending the fragment
static OVERLAY_PAGE: &str = include_str!("../../../../overlay/dist/index.html");

/// GET /overlay
///
/// HTML page for the overlay
pub async fn page() -> impl IntoResponse {
    ([(CONTENT_TYPE, "text/html")], OVERLAY_PAGE)
}
