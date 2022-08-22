//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-templates
//! ```

use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::Deserialize;


// define the templates
#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    username: String,
}

async fn greet(extract::Path(username): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { username };
    HtmlTemplate(template)
}



#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}


#[tokio::main]
async fn main() {

    // build our application with some routes
    let app = Router::new()
        .route("/greet/:username", get(greet));


    // run it
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

