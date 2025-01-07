use axum::{
    body::{Body, Bytes},
    extract::MatchedPath,
    http::{HeaderMap, Request},
};
use std::time::Duration;
use tower_http::{
    classify::{ServerErrorsAsFailures, ServerErrorsFailureClass},
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, info_span, Level, Span};

#[allow(clippy::type_complexity)]
pub fn create_trace_layer() -> TraceLayer<
    tower_http::classify::SharedClassifier<ServerErrorsAsFailures>,
    impl Fn(&Request<Body>) -> Span + Clone,
    impl Fn(&Request<Body>, &Span) + Clone,
    DefaultOnResponse,
    impl Fn(&Bytes, Duration, &Span) + Clone,
    impl Fn(Option<&HeaderMap>, Duration, &Span) + Clone,
    impl Fn(ServerErrorsFailureClass, Duration, &Span) + Clone,
> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(|p| p.as_str());

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            info!("started {} {}", request.method(), request.uri().path())
        })
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        )
        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
            // Body chunk handling
        })
        .on_eos(
            |_trailers: Option<&HeaderMap>, _duration: Duration, _span: &Span| {
                // Stream end handling
            },
        )
        .on_failure(|_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
            // Failure handling
        })
}
