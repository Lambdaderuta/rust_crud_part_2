use std::sync::Arc;

use crate::{
    services::product::product_service,
    structs::product::{CreateProduct, ProductDelete, UpdateProduct},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProduct>,
) -> Result<(), (StatusCode, String)> {
    product_service::create_product(payload, state).await?;

    Ok(())
}

pub async fn get_product_list(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let res = product_service::get_product_list(state).await.unwrap();

    Ok(Json(res))
}

pub async fn update_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateProduct>,
) -> Result<(), (StatusCode, String)> {
    product_service::update_product(payload, state).await?;

    Ok(())
}

pub async fn delete_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ProductDelete>,
) -> Result<(), (StatusCode, String)> {
    product_service::delete_product(payload.id, state).await?;

    Ok(())
}
