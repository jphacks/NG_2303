use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontendData {
    image_label: String,
    image_base64_vec: Vec<ImageData>,
}

impl FrontendData {
    pub fn new(image_label: String, image_base64_vec: Vec<ImageData>) -> Self {
        Self {
            image_label,
            image_base64_vec,
        }
    }
    pub fn new2() -> Self {
        Self {
            image_label: "".to_string(),
            image_base64_vec: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    image_base64: String,
    // 複数の物体が含まれている場合は，複数のラベルを持つ．
    label: Vec<String>,
    forbidden_label: bool,
    noize_info: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectDetectionData {
    image_base64: String,    // 画像データ
    true_label: String,      // 物体の本来のラベル
    predicted_label: String, // 物体の誤認識の結果のラベル
    forbidden_label: bool,   // ユーザが選択してはいけないことを示すラベル（2値）
    noise_info: String,      // どのようなノイズがかかっているかの情報
    objects: Vec<Object>,    // 物体検出結果のオブジェクトのリスト
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    label: String,     // 物体のラベル
    confidence: f32,   // 物体が存在する確信度
    bbox: BoundingBox, // 物体のバウンディングボックス
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
    x_min: f32, // バウンディングボックスの左端のx座標
    y_min: f32, // バウンディングボックスの上端のy座標
    x_max: f32, // バウンディングボックスの右端のx座標
    y_max: f32, // バウンディングボックスの下端のy座標
}

pub(crate) trait DataAccumu {
    fn select(&self) -> Result<Vec<ObjectDetectionData>>;
    fn insert(&self, data: ObjectDetectionData) -> Result<()>;
    fn delete(&self, id: u64) -> Result<()>;
}
