mod config;
mod controllers;
mod middlewares;
mod services;
mod structs;
mod utils;

use std::sync::Arc;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{post, delete, get, put, patch},
    Router, middleware, body::Body,
};
use config::Config;
use controllers::auth::{login, register};
use dotenv::dotenv;
use tokio::signal;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::controllers::product::{create_product, get_product_list, update_product, delete_product};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
            "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
    
    
    dotenv().ok();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let app_state = Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    });

    let user_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let products_routes = Router::new()
        .route("/create", put(create_product))
        .route("/get_list", get(get_product_list))
        .route("/update", patch(update_product))
        .route("/delete", delete(delete_product))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), middlewares::auth::auth::<Body>));

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .nest("/users", user_routes)
        .nest("/products", products_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server started successfully");
    
    axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
