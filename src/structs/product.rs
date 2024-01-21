use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub category: String,
    pub price: i32,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub color: String,
    pub category: String,
    pub price: i32,
}

#[derive(Deserialize)]
pub struct UpdateProduct {
    pub id: i32,
    pub name: Option<String>,
    pub color: Option<String>,
    pub category: Option<String>,
    pub price: Option<i32>,
}

#[derive(Serialize)]
pub struct ProductFromBd {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub category: Option<String>,
    pub price: Option<i32>,
}

#[derive(Deserialize)]
pub struct ProductDelete {
    pub id: i32,
}
