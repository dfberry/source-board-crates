//! Run with
//!
//! ```not_rust
//! cargo run -p example-diesel-postgres
//! ```
//!
//! Checkout the [diesel webpage](https://diesel.rs) for
//! longer guides about diesel
//!
//! Checkout the [crates.io source code](https://github.com/rust-lang/crates.io/)
//! for a real world application using axum and diesel

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::env;
use dotenvy::dotenv;
//use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// this embeds the migrations into the application binary
// the migration path is relative to the `CARGO_MANIFEST_DIR`
//pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub mod route;

use route::read::list;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api-postgres-source-board=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()) != "production" {
        dotenv().ok();
    }
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let port: String= env::var("PORT").expect("PORT environment variable not set");

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/user-configs", get(list))
        //.route("/user/create", post(create_user))
        .with_state(pool);

    // run it with hyper
    let addr = format!("0.0.0.0:{}", port);
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



