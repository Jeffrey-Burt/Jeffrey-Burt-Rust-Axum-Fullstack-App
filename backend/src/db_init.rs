use axum::{
    Router,
};
use sqlx::MySqlPool;
use crate::routes::get_routes;

fn get_database_url() -> String {
    return "mysql://root:Mysqlpassword123@localhost:3306/world".to_string();
}

pub async fn connect_to_db() -> Router {
    let pool = MySqlPool::connect(&get_database_url()).
    await
    .expect("Could not connect to the database");

    return get_routes(pool).await;
}