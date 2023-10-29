use sqlx::PgPool;

use crate::accumu::NoisedImage;

use anyhow::Result;

pub async fn select(object_label: &str, pool: &PgPool) -> Result<Vec<NoisedImage>> {
    let data = sqlx::query_as(
        "SELECT * FROM noised_images WHERE object_label = $1"
    )
    .bind(object_label)
    .fetch_all(pool)
    .await?;

    // let mut data = Vec::new();
    // for row in rows {
    //     let forbidden_label = match row.forbidden_label {
    //         0 => false,
    //         1 => true,
    //         _ => panic!("forbidden_label is not 0 or 1"),
    //     };

    //     let noised_image = NoisedImage::new(
    //         row.image_url,
    //         row.object_label,
    //         row.noise_info,
    //         forbidden_label,
    //     );

    //     data.push(noised_image);
    // }

    Ok(data)
}

pub async fn insert(data: NoisedImage, pool: &PgPool) -> Result<()> {
    sqlx::query(
        "INSERT INTO noised_images 
        (image_url, object_label, noise_info, forbidden_label)
        VALUES ($1, $2, $3, $4)"
    )
    .bind(data.image_url)
    .bind(data.object_label)
    .bind(data.noise_info)
    .bind(data.forbidden_label)
    .execute(pool)
    .await?;

    Ok(())
}
