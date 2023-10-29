use sqlx::PgPool;

use crate::accumu::{NoisedImage, NoisedImageStore};

use anyhow::Result;

pub struct NoisedShuttleSharedDb;

#[async_trait::async_trait]
impl NoisedImageStore for NoisedShuttleSharedDb {
    async fn select(&self, object_label: &str, pool: &PgPool) -> Result<Vec<NoisedImage>> {
        let data = sqlx::query_as("SELECT * FROM noised_images WHERE object_label = $1")
            .bind(object_label)
            .fetch_all(pool)
            .await?;
        Ok(data)
    }
    async fn insert(&self, data: NoisedImage, pool: &PgPool) -> Result<()> {
        sqlx::query(
            "INSERT INTO noised_images 
            (image_url, object_label, noise_info, forbidden_label)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(data.image_url)
        .bind(data.object_label)
        .bind(data.noise_info)
        .bind(data.forbidden_label)
        .execute(pool)
        .await?;

        Ok(())
    }
}
