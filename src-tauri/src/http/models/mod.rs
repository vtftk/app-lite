use serde::{Deserialize, Serialize};

pub mod calibration;

#[derive(Debug, Deserialize)]
pub struct SetAuthTokenRequest {
    pub auth_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetAuthTokenResponse {
    pub auth_token: Option<String>,
}
