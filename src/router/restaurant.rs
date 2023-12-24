use axum::extract::State;
use crate::types::{Restaurant, Category};
use crate::types;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RestaurantQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub category: Option<String>,
}

impl Default for RestaurantQuery {
    fn default() -> Self {
        Self {
            category: None,
            limit: Some(0),
            offset: Some(10),
        }
    }
}

pub mod endpoints {
    use axum::{extract::{Path, Query, State}, Json};

    use crate::{types::{self, Restaurant, Category}, consts};

    use super::RestaurantQuery;
   
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct RestaurantResponse {
        pub restaurants: Vec<Restaurant>,
        pub categories: Vec<Category>,
    }
    
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Business {
        pub id: String,
        pub alias: String,
        pub name: String,
        pub image_url: String,
        pub is_closed: bool,
        pub url: String,
        pub review_count: usize,
        pub categories: Vec<Category>,
        pub rating: f32,
        pub coordinates: serde_json::Value,
        pub transactions: Vec<String>,
        pub price: Option<String>,
        pub location: serde_json::Value,
        pub phone: String,
        pub display_phone: String,
        pub distance: f32,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct RestaurantBusinessResponse {
        pub businesses: Vec<Business>,
    }

    pub async fn restaurants(Path(location): Path<String>, query: Option<Query<RestaurantQuery>>, State(state): State<types::State>) -> Json<RestaurantResponse> {
        let Query(query) = query.unwrap_or_default();

        state.lock().unwrap().restaurants.clear();

        let url = Restaurant::url(location, query.category, query.limit, query.offset);

        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}",consts::API_KEY()))
            .send()
            .await
            .unwrap()
            .json::<RestaurantBusinessResponse>();

        let response = response.await.unwrap();
        
        let businesses = &response.businesses.clone();
        
        for restaurant in businesses {
            let price = match &restaurant.price {
                Some(price) => price,
                None => "N/A",
            };

            Restaurant::new(
                restaurant.name.to_string(),
                restaurant.url.to_string(),
                restaurant.image_url.to_string(),
                price.to_string(),
                restaurant.categories.clone(),
                restaurant.rating,
                State(state.clone()),
            );
        }
        
        let restaurants = state.lock().unwrap().restaurants.clone();
        let categories: Vec<_> = state.lock().unwrap().categories.clone().into_iter().collect();

        Json(RestaurantResponse {
            restaurants,
            categories,
        })
    }
}

impl Restaurant {
    pub fn new(name: String, url: String, image_url: String, price: String, categories: Vec<Category>, rating: f32, State(state): State<types::State>) -> Self {
        
        let restaurant = Self {
            name,
            url,
            image_url,
            price,
            categories,
            rating,
        };

        let mut state = state.lock().unwrap();

        state.restaurants.push(restaurant.clone());

        for category in restaurant.categories.iter() {
            state.categories.insert(category.clone());
        }
        
        restaurant
    }

    pub fn url(location: String, category: Option<String>, limit: Option<usize>, offset: Option<usize> ) -> String {
        format!("https://api.yelp.com/v3/businesses/search?location={}&categories={}&sort_by=best_match&limit={}&offset={}", 
                location, 
                category.unwrap_or_default(),
                limit.unwrap_or_default(),
                offset.unwrap_or_default())
    }

}

#[cfg(test)]
mod tests {
    use std::{sync::Mutex, collections::HashSet};

    use super::*;

    #[test]
    fn test_restaurant_new() {
        let state = types::State::new(Mutex::new( types::SharedState{
            restaurants: Vec::new(),
            categories: HashSet::new(),
        }));

        let categories = vec![
            Category {
                title: String::from("category"),
                alias: String::from("category"),
            },
            Category {
                title: String::from("category"),
                alias: String::from("category") 
            }];

        let restaurant = Restaurant::new(
            "name".to_string(),
            "url".to_string(),
            "image_url".to_string(),
            "price".to_string(),
            categories.clone(),
            4.5,
            axum::extract::State( state.clone() ),
        );

        assert_eq!(restaurant.name, "name");
        assert_eq!(restaurant.url, "url");
        assert_eq!(restaurant.image_url, "image_url");
        assert_eq!(restaurant.price, "price");
        assert_eq!(restaurant.categories, categories);
        assert_eq!(restaurant.rating, 4.5);
        assert_eq!(state.lock().unwrap().restaurants.len(), 1);
        assert_eq!(state.lock().unwrap().categories.len(), 1);

    }
}

