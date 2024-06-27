/**
 * TODO1
 * get mock member list
 * 
 * TODO2
 * get mock member
 * 
 * TODO3 
 * get mock post list 반환
 * 
 * TODO4
 * get mock post
 * 
 **/

use serde::{Deserialize, Serialize};
use axum::{
    routing::{get},
    http::StatusCode,
    Json, Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app: axum::Router = Router::new().route("/users", get(get_users) );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_users() -> (StatusCode, Json<Vec<User>>) {
    let users: Vec<User> = vec![
        User {
            id: 1,
            name: "a".to_string()
        },
        User {
            id: 2,
            name: "b".to_string()
        }
    ];

    (StatusCode::OK, Json(users))
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}