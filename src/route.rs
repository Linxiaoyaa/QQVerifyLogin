use axum::Router;
use axum::routing::post;
use crate::handle::login_verify1;

pub fn  creatroute()-> Router {
          Router::new().route("/verify/submit_ticket",post(login_verify1))
}

