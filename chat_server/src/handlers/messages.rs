use axum::response::IntoResponse;

pub(crate) async fn send_message_handler() -> impl IntoResponse {
    "send messages"
}

pub(crate) async fn list_message_handler() -> impl IntoResponse {
    "list messages"
}
