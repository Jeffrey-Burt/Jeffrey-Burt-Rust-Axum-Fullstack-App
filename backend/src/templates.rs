use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    extract::{Extension, Query},
    Json
};
use crate::api::list_users;
use crate::api::User;
use sqlx::{
    Value,
    MySqlPool,
    types::{JsonValue}
};
use askama::Template;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(T);

#[derive(Template)]
#[template(path = "C:\\Users\\User\\Desktop\\Jeffrey-Burt-Rust-Axum-Fullstack-App\\frontend\\templates\\hello.html")]
pub struct HelloTemplate {
     name: String
}

#[derive(Template)]
#[template(path = "C:\\Users\\User\\Desktop\\Jeffrey-Burt-Rust-Axum-Fullstack-App\\frontend\\templates\\about.html")]
pub struct AboutTemplate {
    button_text: String,
    list: User
}

pub async fn about_template(Extension(pool): Extension<MySqlPool>, Query(user): Query<User>) -> impl IntoResponse {
    let users = list_users(Extension(pool)).await;
    println!("{:?}", users);
    println!("{}", user.name);
    let user = User {
        id: user.id,
        name: user.name,
        email: user.email,
    };
    let template = AboutTemplate { 
        button_text: "Text in button!".to_string(),
        list: user
    };
    HtmlTemplate(template)
}

pub async fn hello() -> impl IntoResponse {
    let template = HelloTemplate { name: "World".to_string() };
    HtmlTemplate(template)
}

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}