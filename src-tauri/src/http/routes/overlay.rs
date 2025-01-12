use axum::response::IntoResponse;
use reqwest::header::CONTENT_TYPE;

/// Embedded oauth response page for handling sending the fragment
static OVERLAY_PAGE: &str = include_str!("../../../../overlay/dist/index.html");
/// Embedded icon for VTube studio
static ICON: &[u8] = include_bytes!("../resources/128x128.png");

/// GET /overlay
///
/// HTML page for the overlay
pub async fn page() -> impl IntoResponse {
    ([(CONTENT_TYPE, "text/html")], OVERLAY_PAGE)
}

/// GET /overlay/icon
///
/// Icon for the overlay to provide to VTube studio when
/// authenticating
pub async fn icon() -> impl IntoResponse {
    ([(CONTENT_TYPE, "image/png")], ICON)
}
