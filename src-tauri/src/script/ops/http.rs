//! # HTTP (JS API)
//!
//! API for performing HTTP requests from within the JS runtime

use anyhow::Context;
use deno_core::*;
use log::debug;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsHttpResponse {
    ok: bool,
    status: u16,
    response: String,
}

/// Operation for performing a GET request to a specific URL from JS
#[op2(async)]
#[serde]
pub async fn op_http_get(#[string] url: String) -> anyhow::Result<JsHttpResponse> {
    debug!("performing http get request: {}", url);

    let response = reqwest::get(url)
        .await
        .context("failed to perform get request")?;

    let status = response.status();
    let body = response.text().await?;
    let ok = status.is_success();

    debug!("http response: {:?}", body);

    Ok(JsHttpResponse {
        ok,
        status: status.as_u16(),
        response: body,
    })
}
