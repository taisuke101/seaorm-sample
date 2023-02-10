// use async_graphql::{
//     http::{playground_source, GraphQLPlaygroundConfig},
//     EmptyMutation, EmptySubscription, Request, Response, Schema,
// };
//use graphql::resolver::QueryRoot;
mod handler;
mod my_error;

use axum::{extract::Extension, routing::get, Router, Server};
use my_error::ServerBuildError;
use sea_orm::Database;
use std::{env, net::SocketAddr, str::FromStr};
use tower::ServiceBuilder;

use handler::{
    family::{create_family, delete_family, get_families, get_family, update_family},
    post::{create_post, delete_post, get_post, get_posts, update_post},
};
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), ServerBuildError> {
    env::set_var("RUST_LOG", "DEBUG");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let host = env::var("HOST")?;
    let port = env::var("PORT")?;
    let server_url = format!("{}:{}", host, port);

    let conn = Database::connect(db_url).await?;
    Migrator::up(&conn, None).await?;

    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route(
            "/family/:id",
            get(get_family).put(update_family).delete(delete_family),
        )
        .route("/family", get(get_families).post(create_family))
        .route(
            "/post/:id",
            get(get_post)
                .post(create_post)
                .put(update_post)
                .delete(delete_post),
        )
        .route("/post", get(get_posts))
        .layer(ServiceBuilder::new().layer(Extension(conn)));

    let addr = SocketAddr::from_str(&server_url)?;
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
