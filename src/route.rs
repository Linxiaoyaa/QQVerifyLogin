use axum::Router;
use axum::routing::post;
use crate::handle::loginverify1;

pub fn  creatroute()-> Router {
          Router::new().route("/verify/submit_ticket",post(loginverify1))
}

