use sqlx::SqlitePool;
use sqlx::Database;
use anyhow::Result;

use crate::accumu::DataAccumu;
use crate::accumu::ObjectDetectionData;

struct DbQuery;

#[async_trait::async_trait]
impl DataAccumu for DbQuery {

async fn select(&self,id:i64,pool: SqlitePool) ->  Result<ObjectDetectionData> {
    let row = sqlx::query!(
        r#"
        SELECT * FROM object_detection_data WHERE id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await?;

    let data = crate::accumu::ObjectDetectionData::new(
        row.base64_image,
        row.label,
        row.image_recognition_result,
        true,
        "".to_string(),
    );

    Ok(data)
    }


async fn insert(&self,data:ObjectDetectionData,pool: SqlitePool) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO object_detection_data 
        (label, image_recognition_result, base64_image)
        VALUES ($1, $2, $3)
        "#,
        data.image_base64,
        data.predicted_label,
        data.noise_info
    )
    .execute(&pool)
    .await?;

    Ok(())
    }


async fn delete(&self,id:i64,pool:SqlitePool) -> Result<()> {
        todo!()
    }
}
