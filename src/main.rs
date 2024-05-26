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

pub mod price;

#[tokio::main]
async fn main() {
    let prices = Mutex::new(HashMap::new());

    let app = Router::new()
        .route("/prices", post(create_price))
        .route("/prices/:id", get(get_price).delete(delete_price))
        .route("/prices", get(get_all_prices))
        .layer(
            CorsLayer::new()
                .allow_origin("*")
                .allow_methods(vec!["GET", "POST", "DELETE"])
                .allow_headers(vec!["authorization", "content-type"]),
        )
        .layer(Extension(prices));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_price(Json(payload): Json<Value>, State(prices): State<Mutex<HashMap<Uuid, Price>>>) {
    let price = payload["price"].as_i64().unwrap();
    let id = Uuid::new_v4();
    let new_price = price::Price { id, price };
    prices.lock().unwrap().insert(id, new_price);
    println!("Created price: {:?}", new_price);
}

async fn get_price(Path(id): Path<Uuid>, State(prices): State<Mutex<HashMap<Uuid, Price>>>) {
    let prices = prices.lock().unwrap();
    if let Some(price) = prices.get(&id) {
        println!("Found price: {:?}", price);
        let json = serde_json::to_string(&price).unwrap();
        axum::Json(json).send().await.unwrap();
    } else {
        println!("Price not found");
        axum::Json(json!({"error": "Price not found"})).send().await.unwrap();
    }
}

async fn delete_price(Path(id): Path<Uuid>, State(prices): State<Mutex<HashMap<Uuid, Price>>>) {
    let prices = prices.lock().unwrap();
    if let Some(_) = prices.remove(&id) {
        println!("Deleted price");
        axum::Json(json!({"status": "ok"})).send().await.unwrap();
    } else {
        println!("Price not found");
        axum::Json(json!({"error": "Price not found"})).send().await.unwrap();
    }
}

async fn get_all_prices(State(prices): State<Mutex<HashMap<Uuid, Price>>>) {
    let prices = prices.lock().unwrap();
    let json = serde_json::to_string(&price::Prices { prices: prices.values().cloned().collect() }).unwrap();
    axum::Json(json).send().await.unwrap();
}

