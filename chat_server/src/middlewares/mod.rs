mod auth;
mod request_id;
mod server_time;

use axum::{middleware::from_fn, Router};
use request_id::set_request_id;
use server_time::ServerTimeLayer;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse},
    LatencyUnit,
};
use tracing::Level;

pub use auth::verify_token;

const REQUEST_ID_HEADER: &str = "x-request-id";
const SERVER_TIME_HEADER: &str = "x-server-time";

pub fn set_layer(app: Router) -> Router {
    app.layer(
        ServiceBuilder::new()
            .layer(
                tower_http::trace::TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            )
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
            .layer(from_fn(set_request_id))
            .layer(ServerTimeLayer),
    )
}
