use std::borrow::Cow;

use twitch_api::helix::Scope;

/// If you are forking this app program for your own use, please create your own
/// twitch developer application client ID at https://dev.twitch.tv/console/apps
pub const TWITCH_CLIENT_ID: &str = "x0zzeitiwvgblu743qnxzaipa9e01z";

/// Scopes required from twitch by the app
pub const TWITCH_REQUIRED_SCOPES: &[Scope] = &[
    // View live Stream Chat and Rooms messages
    Scope::UserReadChat,
    // View Channel Points rewards and their redemptions on your channel.
    Scope::ChannelReadRedemptions,
    // Get a list of all subscribers to your channel and check if a user is subscribed to your channel
    Scope::ChannelReadSubscriptions,
    // View your channel's Bits information
    Scope::BitsRead,
    // Read the list of followers in channels where you are a moderator.
    Scope::Other(Cow::Borrowed("moderator:read:followers")),
];

/// Redirect URI for twitch OAuth, port should match below
pub const TWITCH_REDIRECT_URI: &str = "http://localhost:58371/oauth";

/// Port to run the local server on
pub const LOCAL_SERVER_PORT: u16 = 58371;


