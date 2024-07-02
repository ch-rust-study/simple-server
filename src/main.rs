use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
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
async fn main() {
    tracing_subscriber::fmt::init();

    let app: axum::Router = Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/posts", get(get_posts))
        .route("/posts/:id", get(get_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_users() -> (StatusCode, Json<Vec<User>>) {
    let users: Vec<User> = vec![
        User {
            id: 1,
            name: "a".to_string(),
        },
        User {
            id: 2,
            name: "b".to_string(),
        },
    ];

    (StatusCode::OK, Json(users))
}

async fn get_user(Path(id): Path<u64>) -> (StatusCode, Json<User>) {
    let user = User {
        id,
        name: "tigger".to_string(),
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
    id: u64,
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
