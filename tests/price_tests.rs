use axum::{
    extract::{Json, Path, State},
    response::Json as AxumJson,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::price;

#[tokio::test]
async fn test_create_price() {
    let prices = Mutex::new(HashMap::new());

    let app = Router::new()
        .route("/prices", post(crate::create_price))
        .layer(Extension(prices.clone()));

    let price = price::Price { id: Uuid::new_v4(), price: 100 };
    let payload = json!({"price": 100});

    let response = axum::body::to_bytes(axum::handler::get::<_, axum::body::Body>(
       axum::handler::post(app.into_make_service_with_connect_info::<axum::test_helper::TestConnector>(), "/prices", Json(payload))
   ).await.unwrap())
   .unwrap();

   assert_eq!(response, axum::body::to_bytes(axum::Json(json!({"id": price.id.to_string(), "price": price.price}))).unwrap());

   let prices = prices.lock().unwrap();
   assert!(prices.contains_key(&price.id));
}

#[tokio::test]
async fn test_get_price() {
    let prices = Mutex::new(HashMap::new());

    let app = Router::new()
        .route("/prices/:id", get(crate::get_price))
        .layer(Extension(prices.clone()));

    let price = price::Price { id: Uuid::new_v4(), price: 100 };
    prices.lock().unwrap().insert(price.id, price.clone());

    let response = axum::body::to_bytes(axum::handler::get::<_, axum::body::Body>(
       axum::handler::get(app.into_make_service_with_connect_info::<axum::test_helper::TestConnector>(), &format!("/prices/{}", price.id))
   ).await.unwrap())
   .unwrap();

   assert_eq!(response, axum::body::to_bytes(axum::Json(json!({"id": price.id.to_string(), "price": price.price}))).unwrap());
}

#[tokio::test]
async fn test_delete_price() {
    let prices = Mutex::new(HashMap::new());

    let app = Router::new()
        .route("/prices/:id", delete(crate::delete_price))
        .layer(Extension(prices.clone()));

    let price = price::Price { id: Uuid::new_v4(), price: 100 };
    prices.lock().unwrap().insert(price.id, price.clone());

    let response = axum::body::to_bytes(axum::handler::get::<_, axum::body::Body>(
       axum::handler::delete(app.into_make_service_with_connect_info::<axum::test_helper::TestConnector>(), &format!("/prices/{}", price.id))
   ).await.unwrap())
   .unwrap();

   assert_eq!(response, axum::body::to_bytes(axum::Json(json!({"status": "ok"}))).unwrap());

   let prices = prices.lock().unwrap();
   assert!(!prices.contains_key(&price.id));
}

#[tokio::test]
async fn test_get_all_prices() {
    let prices = Mutex::new(HashMap::new());

    let app = Router::new()
        .route("/prices", get(crate::get_all_prices))
        .layer(Extension(prices.clone()));

    let price1 = price::Price { id: Uuid::new_v4(), price: 100 };
    let price2 = price::Price { id: Uuid::new_v4(), price: 200 };
    prices.lock().unwrap().insert(price1.id, price1.clone());
    prices.lock().unwrap().insert(price2.id, price2.clone());

    let response = axum::body::to_bytes(axum::handler::get::<_, axum::body::Body>(
       axum::handler::get(app.into_make_service_with_connect_info::<axum::test_helper::TestConnector>(), "/prices")
   ).await.unwrap())
   .unwrap();

   let expected_json = serde_json::to_string(&price::Prices { prices: vec![price1, price2] }).unwrap();
   assert_eq!(response, axum::body::to_bytes(axum::Json(json!(expected_json))).unwrap());
}

