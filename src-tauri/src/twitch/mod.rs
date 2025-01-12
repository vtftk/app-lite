use twitch_api::HelixClient;

pub mod manager;
pub mod models;
pub mod websocket;

pub type TwitchClient = HelixClient<'static, reqwest::Client>;
