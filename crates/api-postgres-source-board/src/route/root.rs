use axum::response::IntoResponse;

pub async fn getRoot() -> impl IntoResponse {
    axum::response::Html("<h1>Hello</h1>")
}