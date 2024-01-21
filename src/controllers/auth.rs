use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    services::auth::auth_service,
    structs::auth::{Auth, AuthServicePayload},
    AppState,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Auth>,
) -> Result<(), (StatusCode, String)> {
    auth_service::register(AuthServicePayload { payload, state }).await?;

    Ok(())
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Auth>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let res = auth_service::login(AuthServicePayload { payload, state }).await?;

    Ok(res)
}
