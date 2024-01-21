use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize)]
pub struct Auth {
    pub email: String,
    pub password: String,
}

pub struct AuthServicePayload {
    pub payload: Auth,
    pub state: Arc<AppState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
