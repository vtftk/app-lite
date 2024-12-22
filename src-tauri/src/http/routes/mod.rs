use axum::{
    routing::{get, post, put},
    Router,
};

mod calibration;
mod data;
mod events;
mod oauth;
mod overlay;

pub fn router() -> Router {
    Router::new()
        .route("/oauth", get(oauth::handle_oauth))
        .route("/oauth/complete", post(oauth::handle_oauth_complete))
        .route("/events", get(events::handle_sse))
        .route(
            "/calibration",
            post(calibration::handle_calibration_progress),
        )
        .route(
            "/calibration-data",
            get(calibration::handle_calibration_data),
        )
        .route("/app-data", get(data::get_app_data))
        .route("/runtime-app-data", get(data::get_runtime_data))
        .route("/runtime-app-data", put(data::update_runtime_data))
        .route("/content/:folder/:name", get(data::get_content_file))
        .route("/defaults/:folder/:name", get(data::get_defaults_file))
        .route("/overlay", get(overlay::page))
        .route("/overlay/icon", get(overlay::icon))
        .route("/data/get-auth-token", get(data::handle_get_auth_token))
        .route("/data/set-auth-token", post(data::handle_set_auth_token))
}
