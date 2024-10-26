// Rust
use std::env;
use dotenvy::dotenv;
use tokio::sync::OnceCell;
use serde::{Serialize, Deserialize};

// Web server
use axum::{
    Router,
    routing::{get, post, delete},
    response::{
        IntoResponse, 
        Response
    }, 
    http::StatusCode,
    body::Body,
    extract::{Query, Json, Extension, State},
};

// Database
mod schema;
use schema::book::dsl::*;
use diesel::{dsl::insert_into, prelude::Queryable, Insertable, Selectable};
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection, RunQueryDsl, AsyncConnection
};


#[derive(Insertable)]
#[diesel(table_name = schema::book)]
struct NewBook {
    title: String,
}
#[derive(Deserialize, Serialize, Queryable)]
#[diesel(table_name = schema::book)]
struct Book {
    id: i32,
    title: String,
}

async fn build_connection_pool() -> Pool<AsyncPgConnection> {
    dotenv().ok();
    let connection_url = env::var("DATABASE_URL").unwrap();
    print!("Connecting to {}", connection_url);

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_url);
    Pool::builder().build(manager).await.unwrap()
}

async fn get_connection_pool() -> &'static Pool<AsyncPgConnection> {
    static POOL: OnceCell<Pool<AsyncPgConnection>> = OnceCell::const_new();
    POOL.get_or_init(build_connection_pool).await
}


async fn hello_world() -> &'static str {
    "Hello, World!"
}
#[derive(Deserialize)]
struct AddBookQuery {
    title: String,
}
async fn add_book(
    State(pool): State<Pool<AsyncPgConnection>>,
    Json(params): Json<AddBookQuery>
) ->  impl IntoResponse {

    let conn = &mut pool.get().await.unwrap();

    let new_book = NewBook { title: params.title };

    let inserted_book: Book = diesel::insert_into(book)
        .values(&new_book)
        .get_result(conn)
        .await
        .unwrap();

    Json(inserted_book)

}

async fn get_all_books(
    State(pool): State<Pool<AsyncPgConnection>>
) -> impl IntoResponse {
    use schema::book::dsl::*;

    let conn = &mut pool.get().await.unwrap();

    let results = book
        .load::<Book>(conn)
        .await
        .unwrap();

    Json(results)
}
#[derive(Deserialize)]
struct DeleteBookQuery {
    id: i32,
}
async fn delete_book(
    State(pool): State<Pool<AsyncPgConnection>>,
    Query(params): Query<DeleteBookQuery>
) -> impl IntoResponse {

    let conn = &mut pool.get().await.unwrap();

    let deleted_book: Book = diesel::delete(book.filter(id.eq(params.id)))
        .get_result(conn)
        .await
        .unwrap();

    Json(deleted_book)
}

#[tokio::main]
async fn main() {

    if env::var("ENVIRONMENT").unwrap_or_else(|_| "".to_string()).to_lowercase() == "development" {
        dotenv().ok();
    }

    let connection_url = env::var("DATABASE_URL").unwrap();
    print!("Connecting to {}", connection_url);

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(connection_url);
    let pool = Pool::builder().build(config).await.unwrap();

    let port = env::var("PORT").unwrap();
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/book", post(add_book))
        .route("/book/{id}", delete(delete_book))
        .route("/books", get(get_all_books))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
