use std::{sync::{Mutex, Arc}, collections::HashSet};

use axum::{Router, routing::get, http::{Method, header::CONTENT_TYPE, HeaderValue}};
use tower_http::cors::{CorsLayer, Any};

use crate::types::{State, SharedState};

pub mod restaurant;

pub fn router() -> Router {
    let state = State::new(Mutex::new( SharedState{
        restaurants: Vec::new(),
        categories: HashSet::new(),
    }));

    let router = Router::new();
    //let origins = [
     //   "localhost:5173".parse().unwrap(),
       // "https://www.restaurant.edlingao.me".parse().unwrap(),
    //];
    router
        .route("/", get(|| async { "Hello, World!" }))
        .route("/restaurants/:location", get(restaurant::endpoints::restaurants))
        .with_state(Arc::clone(&state))
        .layer(CorsLayer::permissive())
}

