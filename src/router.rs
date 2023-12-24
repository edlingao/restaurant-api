use std::{sync::{Mutex, Arc}, collections::HashSet};

use axum::{Router, routing::get};

use crate::types::{State, SharedState};

pub mod restaurant;

pub fn router() -> Router {
    let state = State::new(Mutex::new( SharedState{
        restaurants: Vec::new(),
        categories: HashSet::new(),
    }));

    let router = Router::new();
    
    router
        .route("/", get(|| async { "Hello, World!" }))
        .route("/restaurants/:location", get(restaurant::endpoints::restaurants))
        .with_state(Arc::clone(&state))
}

