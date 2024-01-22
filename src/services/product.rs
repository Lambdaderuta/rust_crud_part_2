pub mod product_service {
    use std::sync::Arc;

    use axum::http::StatusCode;

    use crate::{
        structs::product::{CreateProduct, ProductFromBd, UpdateProduct},
        utils::connection::internarnal_error,
        AppState,
    };

    pub async fn get_product_list(
        state: Arc<AppState>,
    ) -> Result<Vec<ProductFromBd>, (StatusCode, String)> {
        let res = sqlx::query_as!(ProductFromBd, "SELECT * FROM products")
            .fetch_all(&state.db)
            .await
            .map_err(internarnal_error)
            .unwrap();

        Ok(res)
    }

    pub async fn create_product(
        payload: CreateProduct,
        state: Arc<AppState>,
    ) -> Result<(), (StatusCode, String)> {
        let _ = sqlx::query!(
            "INSERT INTO products (name, color, category, price) VALUES ($1, $2, $3, $4)",
            payload.name,
            payload.color,
            payload.category,
            payload.price,
        )
        .execute(&state.db)
        .await;

        Ok(())
    }

    pub async fn update_product(
        payload: UpdateProduct,
        state: Arc<AppState>,
    ) -> Result<(), (StatusCode, String)> {
        let _ = sqlx::query!(
            "UPDATE products
            SET name=coalesce($2, name),
            color=coalesce($3, color),
            category=coalesce($4, category),
            price=coalesce($5, price)
            WHERE id = $1",
            payload.id,
            payload.name,
            payload.color,
            payload.category,
            payload.price
        )
        .execute(&state.db)
        .await;

        Ok(())
    }

    pub async fn delete_product(
        payload: i32,
        state: Arc<AppState>,
    ) -> Result<(), (StatusCode, String)> {
        let _ = sqlx::query!("DELETE FROM products WHERE id = $1", payload)
            .execute(&state.db)
            .await;

        Ok(())
    }
}
