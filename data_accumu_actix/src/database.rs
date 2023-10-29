use anyhow::Result;
use sqlx::any::AnyRow;
use sqlx::Database;
use sqlx::SqlitePool;

use crate::accumu::DataAccumu;
use crate::accumu::DataStore;
use crate::accumu::NoisedImage;
use crate::accumu::ObjectDetectionData;

struct DbQuery;

#[async_trait::async_trait]
impl DataAccumu for DbQuery {
    async fn select(&self, id: i64, pool: SqlitePool) -> Result<ObjectDetectionData> {
        let row = sqlx::query!(
            r#"
        SELECT * FROM object_detection_data WHERE id = ?
        "#,
            id
        )
        .fetch_one(&pool)
        .await?;

        let forbidden_label = match row.forbidden_label {
            0 => false,
            1 => true,
            _ => panic!("forbidden_label is not 0 or 1"),
        };

        let data = crate::accumu::ObjectDetectionData::new(
            row.image_url,
            row.object_label,
            row.predicted_label,
            row.confidence,
            forbidden_label,
            row.noise_info,
        );

        Ok(data)
    }

    async fn insert(&self, data: ObjectDetectionData, pool: SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"
        INSERT INTO object_detection_data 
        (image_url, object_label, predicted_label, confidence, forbidden_label, noise_info)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
            data.image_url,
            data.object_label,
            data.predicted_label,
            data.confidence,
            data.forbidden_label,
            data.noise_info
        )
        .execute(&pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i64, pool: SqlitePool) -> Result<()> {
        todo!()
    }
}

#[async_trait::async_trait]
impl DataStore for NoisedImage {
    async fn select(&self, object_label: &str, pool: SqlitePool) -> Result<Vec<NoisedImage>> {
        let rows = sqlx::query!(
            r#"
        SELECT * FROM noised_images WHERE object_label = ?
        "#,
            object_label
        )
        .fetch_all(&pool)
        .await?;

        let mut data = Vec::new();
        for row in rows {
            let forbidden_label = match row.forbidden_label {
                0 => false,
                1 => true,
                _ => panic!("forbidden_label is not 0 or 1"),
            };

            let noised_image = NoisedImage::new(
                row.image_url,
                row.object_label,
                row.noise_info,
                forbidden_label,
            );

            data.push(noised_image);
        }

        Ok(data)
    }

    async fn insert(&self, data: NoisedImage, pool: SqlitePool) -> Result<()> {

        let forbidden_label = match data.forbidden_label {
            false => 0,
            true => 1,
        };
        sqlx::query!(
            r#"
            INSERT INTO noised_images 
            (image_url, object_label, noise_info, forbidden_label)
            VALUES ($1, $2, $3, $4)
            "#,
            data.image_url,
            data.object_label,
            data.noise_info,
            forbidden_label
        )
        .execute(&pool)
        .await?;

        Ok(())
    }
}
