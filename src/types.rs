use std::{sync::{Arc, Mutex}, collections::HashSet};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Restaurant {
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub price: String,
    pub categories: Vec<Category>,
    pub rating: f32,
}

#[derive(Debug, Clone)]
pub struct SharedState {
    pub restaurants: Vec<Restaurant>,
    pub categories: HashSet<Category>,
}

pub type State = Arc<Mutex<SharedState>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Category {
    pub alias: String,
    pub title: String,
}

