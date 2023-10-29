use sqlx::PgPool;

use crate::accumu::{ObjectDetectionData, ObjectDetectionDataStore};

use anyhow::Result;

pub struct ObjectShuttleSharedDb;

#[async_trait::async_trait]
impl ObjectDetectionDataStore for ObjectShuttleSharedDb {
    async fn select(&self, id: i64, pool: &PgPool) -> Result<ObjectDetectionData> {
        let row = sqlx::query_as("SELECT * FROM object_detection_data WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(row)
    }

    async fn insert(&self, data: ObjectDetectionData, pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO object_detection_data 
        (image_url, object_label, predicted_label, confidence, forbidden_label, noise_info)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        )
        .bind(data.image_url)
        .bind(data.object_label)
        .bind(data.predicted_label)
        .bind(data.confidence)
        .bind(data.forbidden_label)
        .bind(data.noise_info)
        .execute(pool)
        .await?;

        Ok(())
    }

}