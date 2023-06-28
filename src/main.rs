use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct Greeting {
    value: Mutex<String>,
}

async fn greet(State(greeting): State<Arc<Greeting>>) -> Json<Value> {
    let name = "Marcus Rådell";
    let greeting = greeting.value.lock().unwrap().clone();
    Json(json!({ "greeting": format!("{greeting} {name}") }))
}

async fn set_greeting(State(greeting): State<Arc<Greeting>>) {
    let mut data = greeting.value.lock().unwrap();
    *data = "Oi!".to_string();
}

#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum {
    let shared_state = Arc::new(Greeting {
        value: Mutex::new("Tja,".to_string()),
    });
    let app = Router::new()
        .route("/", get(greet))
        .route("/", post(set_greeting))
        .with_state(shared_state);
    Ok(app.into())
}
