use std::sync::Arc;

use axum::{
    body::Body,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    controllers::{
        auth::{login, register},
        product::{create_product, delete_product, get_product_list, update_product},
    },
    middlewares, AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let user_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let products_routes = Router::new()
        .route("/create", put(create_product))
        .route("/get_list", get(get_product_list))
        .route("/update", patch(update_product))
        .route("/delete", delete(delete_product))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            middlewares::auth::auth::<Body>,
        ));

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    Router::new()
        .nest("/users", user_routes)
        .nest("/products", products_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
