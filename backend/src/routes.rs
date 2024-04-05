use axum::{
    extract::Extension,
    middleware::self,
    routing::{get, post, delete},
    Router,
};
use sqlx::MySqlPool;
use crate::logging::logging_middleware;
use crate::api::*;
use crate::templates::*;

pub async fn get_routes(pool: MySqlPool) -> Router {
    return Router::new()
    .route("/", get(|| async {"Hello, Rust!"}))
    .route("/hello-world", get(hello))
    .route("/about", get(about_template))
    .route("/new_user", get(new_user_template))
    .route("/api/users", get(get_users))
    .route("/api/item/:id", get(show_item))
    .route("/api/create-user", post(create_user))
    .route("/api/add-item", post(add_item))
    .route("/api/remove-user/:user_id", post(remove_user))
    .route("/api/add-user/:user_id", post(add_user))
    .route("/api/delete-user/:user_id", delete(delete_user))
    .layer(Extension(pool))
    .layer(middleware::from_fn(logging_middleware))
}