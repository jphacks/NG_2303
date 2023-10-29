use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

/// フロント，バック間で送信，受信されるデータ型
#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct BeJudgeImages {
    pub object_label: String, //be_judge_imagesに含まれる画像のラベル 1種類しかないはずなので，ここにも持ってきた
    pub noized_images: Vec<NoisedImage>,
}
impl BeJudgeImages {
    pub fn new(object_label: String, noized_images: Vec<NoisedImage>) -> Self {
        Self {
            object_label,
            noized_images,
        }
    }

    pub fn new_dammy() -> Self {
        Self {
            object_label: "".to_string(),
            noized_images: vec![NoisedImage::new_dammy()],
        }
    }
}

/// 画像1つあたりのデータ型
/// DBにもこの型で保存する
#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct NoisedImage {
    pub image_url: String, // Amazon S3からのURL
    //image_base64: String,になる可能性もある
    pub object_label: String,
    pub noise_info: String, //どのような構造の情報が入るのか決定できないためString
    pub forbidden_label: bool, // ユーザが選択した場合，botの可能性を疑うことを示すラベル
}

impl NoisedImage {
    pub fn new(
        image_url: String,
        object_label: String,
        noise_info: String,
        forbidden_label: bool,
    ) -> Self {
        Self {
            image_url,
            object_label,
            noise_info,
            forbidden_label,
        }
    }

    pub fn new_dammy() -> Self {
        Self {
            image_url: "".to_string(),
            object_label: "".to_string(),
            noise_info: "".to_string(),
            forbidden_label: false,
        }
    }
}

// うまくいかないので設計書みたいに使われてるだけ
#[async_trait]
pub trait NoisedImageStore {
    async fn select(&self, object_label: &str, pool: &PgPool) -> Result<Vec<NoisedImage>>;
    async fn insert(&self, data: NoisedImage, pool: &PgPool) -> Result<()>;
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ObjectDetectionData {
    pub image_url: String,       // Amazon S3からのURL
    pub object_label: String,    // 物体の本来のラベル
    pub predicted_label: String, // 物体の誤認識の結果のラベル
    pub confidence: f64,         // 物体が存在する確信度
    pub forbidden_label: bool,   // ユーザが選択してはいけないことを示すラベル（2値）
    pub noise_info: String,      // どのようなノイズがかかっているかの情報
}

impl ObjectDetectionData {
    pub fn new(
        image_url: String,
        object_label: String,
        predicted_label: String,
        confidence: f64,
        forbidden_label: bool,
        noise_info: String,
    ) -> Self {
        Self {
            image_url,
            object_label,
            predicted_label,
            confidence,
            forbidden_label,
            noise_info,
        }
    }
}

// うまくいかないので設計書みたいに使われてるだけ
#[async_trait]
pub trait ObjectDetectionDataStore {
    async fn select(&self, id: i64, pool: &PgPool) -> Result<ObjectDetectionData>;
    async fn insert(&self, data: ObjectDetectionData, pool: &PgPool) -> Result<()>;
    // async fn delete(&self, id: i64, pool: &PgPool) -> Result<()>;
}

// 少なくともこの2日の期間中は廃止するデータ型
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Object {
//     label: String,     // 物体のラベル
//     confidence: f32,   // 物体が存在する確信度
//     bbox: BoundingBox, // 物体のバウンディングボックス
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BoundingBox {
//     x_min: f32, // バウンディングボックスの左端のx座標
//     y_min: f32, // バウンディングボックスの上端のy座標
//     x_max: f32, // バウンディングボックスの右端のx座標
//     y_max: f32, // バウンディングボックスの下端のy座標
// }
