use axum::http::StatusCode;

pub trait Validate {
    fn validate(&self) -> anyhow::Result<(), (StatusCode, String)>;
}