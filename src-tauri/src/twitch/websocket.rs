use anyhow::{anyhow, bail, Context};
use log::{info, warn};
use tokio_tungstenite::tungstenite;
use twitch_api::{
    eventsub::{
        self,
        event::websocket::{EventsubWebsocketData, ReconnectPayload, SessionData, WelcomePayload},
        Event,
    },
    twitch_oauth2::{TwitchToken, UserToken},
    HelixClient,
};

pub struct WebsocketClient {
    /// The session id of the websocket connection
    pub session_id: Option<String>,
    /// The token used to authenticate with the Twitch API
    pub token: UserToken,
    /// The client used to make requests to the Twitch API
    pub client: HelixClient<'static, reqwest::Client>,
    /// The url to use for websocket
    pub connect_url: url::Url,
}

impl WebsocketClient {
    pub fn new(
        session_id: Option<String>,
        token: UserToken,
        client: HelixClient<'static, reqwest::Client>,
    ) -> Self {
        Self {
            session_id,
            token,
            client,
            connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
        }
    }

    /// Run the websocket subscriber
    pub async fn run(mut self) -> anyhow::Result<()> {
        // Establish the stream
        let mut s = self
            .connect()
            .await
            .context("when establishing connection")?;
        // Loop over the stream, processing messages as they come in.
        loop {
            tokio::select!(
            Some(msg) = futures::StreamExt::next(&mut s) => {
                let msg = match msg {
                    Err(tungstenite::Error::Protocol(
                        tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                    )) => {
                        warn!(
                            "connection was sent an unexpected frame or was reset, reestablishing it"
                        );
                        s = self
                            .connect()
                            .await
                            .context("when reestablishing connection")?;
                        continue
                    }
                    _ => msg?,
                };
                self.process_message(msg).await?
            })
        }
    }

    /// Connect to the websocket and return the stream
    async fn connect(
        &self,
    ) -> anyhow::Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    > {
        info!("connecting to twitch");
        let config = tungstenite::protocol::WebSocketConfig {
            max_message_size: Some(64 << 20), // 64 MiB
            max_frame_size: Some(16 << 20),   // 16 MiB
            accept_unmasked_frames: false,
            ..tungstenite::protocol::WebSocketConfig::default()
        };
        let (socket, _) = tokio_tungstenite::connect_async_with_config(
            self.connect_url.as_str(),
            Some(config),
            false,
        )
        .await
        .context("Can't connect")?;

        info!("connected to twitch event sub");

        Ok(socket)
    }

    /// Process a message from the websocket
    async fn process_message(&mut self, msg: tungstenite::Message) -> anyhow::Result<()> {
        match msg {
            tungstenite::Message::Text(s) => {
                info!("twitch message: {s}");
                match Event::parse_websocket(&s)? {
                    EventsubWebsocketData::Welcome {
                        payload: WelcomePayload { session },
                        ..
                    }
                    | EventsubWebsocketData::Reconnect {
                        payload: ReconnectPayload { session },
                        ..
                    } => {
                        self.process_welcome_message(session).await?;
                        Ok(())
                    }
                    // Here is where you would handle the events you want to listen to
                    EventsubWebsocketData::Notification {
                        metadata: _,
                        payload,
                    } => {
                        println!("{:?}", payload);
                        Ok(())
                    }
                    EventsubWebsocketData::Revocation {
                        metadata,
                        payload: _,
                    } => bail!("got revocation event: {metadata:?}"),
                    EventsubWebsocketData::Keepalive {
                        metadata: _,
                        payload: _,
                    } => Ok(()),
                    _ => Ok(()),
                }
            }
            tungstenite::Message::Close(_) => todo!(),
            _ => Ok(()),
        }
    }

    async fn process_welcome_message(&mut self, data: SessionData<'_>) -> anyhow::Result<()> {
        self.session_id = Some(data.id.to_string());
        if let Some(url) = data.reconnect_url {
            self.connect_url = url.parse()?;
        }

        if self.token.is_elapsed() {
            // TODO: Token is expired Request user to add new token
            return Err(anyhow!("Token is expired"));
        }

        let transport = eventsub::Transport::websocket(data.id.clone());
        self.client
            .create_eventsub_subscription(
                eventsub::channel::ChannelPointsCustomRewardRedemptionAddV1::broadcaster_user_id(
                    self.token.user_id.clone(),
                ),
                transport.clone(),
                &self.token,
            )
            .await?;

        Ok(())
    }
}
