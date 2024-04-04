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
use serde_json::{Result, Value as Serde_Value};

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
}

pub async fn about_template(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let users = list_users(Extension(pool)).await;
    //println!("{:?}", users);

    //Get rid of the json at the beginning, try to make it just <Vec<JsonValue>>

    for i in users {

    };
    println!("{}", users.unwrap());
    println!("{:?}", *users.unwrap());
    let v: Serde_Value = serde_json::from_value(users).unwrap();
    //let v: Value = serde_json::from(users);
    //println!("{} {} {}", v["id"], v["name"], v["email"]);
    let template = AboutTemplate { 
        button_text: "Text in button!".to_string()
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