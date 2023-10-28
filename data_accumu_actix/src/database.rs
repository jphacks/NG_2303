use sqlx::{SqlitePool};

use crate::accumu::DataAccumu;

struct database;


impl DataAccumu for database {
    fn select(&self, id: u64, pool: impl sqlx::Database) -> anyhow::Result<crate::accumu::ObjectDetectionData> {
        todo!()
    }

    fn insert(&self, data: crate::accumu::ObjectDetectionData, pool: impl sqlx::Database) -> anyhow::Result<()> {
        todo!()
    }

    fn delete(&self, id: u64, pool: impl sqlx::Database) -> anyhow::Result<()> {
        todo!()
    }
}

impl database{
    async fn select(&self, id: i64, pool: SqlitePool) -> anyhow::Result<crate::accumu::ObjectDetectionData> {
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
    
    /// ユーザが選択した画像を物体検出にかけた結果をデータベースに保存する．
    // 未完成
    async fn insert(&self, data: crate::accumu::ObjectDetectionData, pool: SqlitePool) -> anyhow::Result<()> {
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
    
    // async fn delete(&self, id: u64, pool: impl sqlx::Database) -> anyhow::Result<()> {
    //     todo!()
    // }
}
