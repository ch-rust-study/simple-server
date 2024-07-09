use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

/**
 * TODO1 @andrew
 * get mock member list
 *
 * TODO2 @tigger
 * get mock member
 *
 * TODO3 @andrew
 * get mock post list 반환
 *
 * TODO4 @tigger
 * get mock post
 *
 **/
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://mb-188:password@localhost/mydb")
        .await?;

    let app: axum::Router = Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/posts", get(get_posts))
        .route("/posts/:id", get(get_post))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_users(State(pool): State<PgPool>) -> (StatusCode, Json<Vec<User>>) {
    let rows: Vec<(i32, String)> = sqlx::query_as("SELECT * from \"USER\"")
        .fetch_all(&pool)
        .await
        .expect("error");

    let users = rows
        .iter()
        .map(|r| User {
            id: r.0,
            name: r.1.to_string(),
        })
        .collect();

    (StatusCode::OK, Json(users))
}

async fn get_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> (StatusCode, Json<User>) {
    let row: (i32, String) = sqlx::query_as("SELECT * from \"USER\" WHERE id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("error");

    let user = User {
        id: row.0,
        name: row.1.to_string(),
    };

    (StatusCode::OK, Json(user))
}

async fn get_posts() -> (StatusCode, Json<Vec<Post>>) {
    let posts: Vec<Post> = vec![Post {
        id: 1,
        title: "a".to_string(),
        author_id: 1,
        content: "a".to_string(),
        created_at: 0,
        updated_at: 0,
    }];

    (StatusCode::OK, Json(posts))
}

async fn get_post(Path(id): Path<u64>) -> (StatusCode, Json<Post>) {
    let post = Post {
        id,
        title: "a".to_string(),
        author_id: 1,
        content: "a".to_string(),
        created_at: 0,
        updated_at: 0,
    };

    (StatusCode::OK, Json(post))
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct Post {
    id: u64,
    title: String,
    author_id: u64,
    content: String,
    created_at: u64,
    updated_at: u64,
}
