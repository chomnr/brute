use actix::Addr;
use axum::{routing::get, Extension, Router};

use crate::brute::Brute;

pub fn get_router() -> Router {
    Router::new().route("/test", get(plain_text))
}

async fn plain_text(Extension(actor): Extension<Addr<Brute>>) -> &'static str {
    //actor.tell("testing.", None);
    "foo"
}
