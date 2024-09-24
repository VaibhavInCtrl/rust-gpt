use crate::db::models::Item;
use sqlx::PgPool;

pub async fn get_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as!(Item, "SELECT id, name FROM items")
        .fetch_all(pool)
        .await?;
    Ok(items)
}