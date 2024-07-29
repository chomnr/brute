use axum::{routing::get, Router};

pub fn get_router() -> Router {
    Router::new().route("/test", get(plain_text))
}

async fn plain_text() -> &'static str {
    //actor.tell("testing.", None);
    "foo"
}
