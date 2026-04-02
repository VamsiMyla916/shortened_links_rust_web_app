use askama::Template;
use axum::{
    extract::{Path, State, Form},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;

// 1. ASKAMA TEMPLATE SETUP
// This connects your Rust struct to your index.html file
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    error_message: Option<String>,
}

// 2. FORM DATA SETUP
// This tells Rust what data to expect when the user clicks "Submit"
#[derive(Deserialize)]
struct LinkFormData {
    original_url: String,
    rusty_alias: String,
}

// 3. THE MAIN FUNCTION
#[tokio::main]
async fn main() {
    // Connect to the SQLite database
    let pool = SqlitePool::connect("sqlite://rusty.db")
        .await
        .expect("Failed to connect to the database");

    // Build the Axum Router
    let app = Router::new()
        .route("/", get(show_form))
        .route("/shorten", post(create_shortlink))
        .route("/:alias", get(redirect_link)) // The ':' means this is a variable!
        .with_state(pool); // Share the database connection with our handlers

    println!("Server running on http://localhost:3000");

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// AXUM HANDLERS
// HANDLER 1: Show the home page
async fn show_form() -> impl IntoResponse {
    let template = IndexTemplate { error_message: None };
    Html(template.render().unwrap())
}

// HANDLER 2: Save the URL to the database
async fn create_shortlink(
    State(pool): State<SqlitePool>,
    Form(data): Form<LinkFormData>,
) -> impl IntoResponse {
    // Try to insert the data into the database
    let result = sqlx::query!(
        "INSERT INTO links (original_url, rusty_alias) VALUES (?, ?)",
        data.original_url,
        data.rusty_alias
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            // Success! Show them their new link.
            let success_html = format!(
                "<h1>Success!</h1><p>Your short link is: <a href='/{0}'>localhost:3000/{0}</a></p>",
                data.rusty_alias
            );
            Html(success_html).into_response()
        }
        Err(_) => {
            // Error! The UNIQUE constraint failed because the word is taken.
            // Re-render the form, but this time pass an error message.
            let template = IndexTemplate {
                error_message: Some(format!("Sorry, the word '{}' is already taken!", data.rusty_alias)),
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

// HANDLER 3: Catch the short link and redirect them
async fn redirect_link(
    Path(alias): Path<String>, // Grab the word from the URL bar
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    // Search the database for the custom word
    let record = sqlx::query!(
        "SELECT original_url FROM links WHERE rusty_alias = ?",
        alias
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    match record {
        Some(row) => {
            // We found it! Redirect the user to the real website.
            Redirect::temporary(&row.original_url).into_response()
        }
        None => {
            // Word not found in the database.
            Html("<h1>404 - Rusty word not found!</h1>".to_string()).into_response()
        }
    }
}