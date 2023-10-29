use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Database, SqlitePool};

/// フロント，バック間で送信，受信されるデータ型
#[derive(Serialize, Deserialize, Debug)]
struct BeJudgeImages {
    object_label: String, //be_judge_imagesに含まれる画像のラベル 1種類しかないはずなので，ここにも持ってきた
    noized_images: Vec<NoizedImage>,
}
impl BeJudgeImages {
    pub fn new(object_label: String, noized_images: Vec<NoizedImage>) -> Self {
        Self {
            object_label,
            noized_images,
        }
    }

    pub fn new_dammy() -> Self {
        Self {
            object_label: "".to_string(),
            noized_images: vec![NoizedImage::new_dammy()],
        }
    }
}

/// 画像1つあたりのデータ型
/// DBにもこの型で保存する
#[derive(Serialize, Deserialize, Debug)]
pub struct NoizedImage {
    image_url: String, // Amazon S3からのURL
    //image_base64: String,になる可能性もある
    object_label: String,
    noize_info: String,    //どのような構造の情報が入るのか決定できないためString
    forbidden_label: bool, // ユーザが選択した場合，botの可能性を疑うことを示すラベル
}

impl NoizedImage {
    pub fn new(
        image_url: String,
        object_label: String,
        noize_info: String,
        forbidden_label: bool,
    ) -> Self {
        Self {
            image_url,
            object_label,
            noize_info,
            forbidden_label,
        }
    }

    pub fn new_dammy() -> Self {
        Self {
            image_url: "".to_string(),
            object_label: "".to_string(),
            noize_info: "".to_string(),
            forbidden_label: false,
        }
    }
}

#[async_trait]
pub(crate) trait DataStore {
    async fn select(&self, object_label: &str, pool: SqlitePool) -> Result<ObjectDetectionData>;
    async fn insert(&self, data: NoizedImage, pool: SqlitePool) -> Result<()>;
    async fn delete(&self, id: i64, pool: SqlitePool) -> Result<()>;
}

#[derive(Serialize, Deserialize, Debug)]
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

/// DIのためのtrait
#[async_trait]
pub(crate) trait DataAccumu {
    async fn select(&self, id: i64, pool: SqlitePool) -> Result<ObjectDetectionData>;
    async fn insert(&self, data: ObjectDetectionData, pool: SqlitePool) -> Result<()>;
    async fn delete(&self, id: i64, pool: SqlitePool) -> Result<()>;
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
