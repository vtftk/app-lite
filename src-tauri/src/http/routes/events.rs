use std::convert::Infallible;

use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    Extension,
};
use futures::Stream;
use tokio_stream::wrappers::BroadcastStream;

use crate::events::EventRecvHandle;

pub async fn handle_sse(
    Extension(event_handle): Extension<EventRecvHandle>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    use tokio_stream::StreamExt;

    let stream = BroadcastStream::new(event_handle.0);
    let stream = stream.filter_map(|result| {
        result
            .ok()
            .and_then(|event| Event::default().json_data(event).ok())
            .map(Ok)
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
