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
use tracing::{error, info, info_span, Level, Span};

fn make_request_span(request: &Request<Body>) -> Span {
    let request_id = uuid::Uuid::new_v4();
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str());

    info_span!(
        "http_request",
        request_id = %request_id,
        method = ?request.method(),
        matched_path,
        some_other_field = tracing::field::Empty,
    )
}

fn handle_request(request: &Request<Body>, _span: &Span) {
    info!("started {} {}", request.method(), request.uri().path())
}

fn handle_body_chunk(chunk: &Bytes, latency: Duration, _span: &Span) {
    info!(
        chunk_size = chunk.len(),
        chunk_latency_ms = latency.as_millis(),
        "Received body chunk"
    );
}

fn handle_eos(trailers: Option<&HeaderMap>, duration: Duration, _span: &Span) {
    info!(
        total_duration_ms = duration.as_millis(),
        has_trailers = trailers.is_some(),
        "Stream completed"
    );
}

fn handle_failure(error: ServerErrorsFailureClass, _latency: Duration, _span: &Span) {
    error!("Server responded with a failure: {}", error)
}

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
        .make_span_with(Box::new(make_request_span))
        .on_request(Box::new(handle_request))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        )
        .on_body_chunk(Box::new(handle_body_chunk))
        .on_eos(Box::new(handle_eos))
        .on_failure(Box::new(handle_failure))
}
