use sqlx::PgPool;

use crate::accumu::ObjectDetectionData;

use anyhow::Result;

pub async fn select(id: i64, pool: &PgPool) -> Result<ObjectDetectionData> {
    let row = sqlx::query_as("SELECT * FROM object_detection_data WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    // let forbidden_label = match row.forbidden_label {
    //     0 => false,
    //     1 => true,
    //     _ => panic!("forbidden_label is not 0 or 1"),
    // };

    // let data = crate::accumu::ObjectDetectionData::new(
    //     row.image_url,
    //     row.object_label,
    //     row.predicted_label,
    //     row.confidence,
    //     forbidden_label,
    //     row.noise_info,
    // );

    // Ok(data);
    Ok(row)
}

pub async fn insert(data: ObjectDetectionData, pool: &PgPool) -> Result<()> {
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
