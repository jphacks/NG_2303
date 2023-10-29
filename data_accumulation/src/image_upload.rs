use actix_web::{post, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use crate::{accumu::NoisedImage, AppState};
use crate::database::{noised_image::NoisedShuttleSharedDb, object_detected::ObjectShuttleSharedDb};
use crate::accumu::{NoisedImageStore, ObjectDetectionDataStore};

/// アップロードする画像を含め必要なデータ
/// 画像はbase64でエンコードされている．
/// どのようなノイズをかけたか，などの情報も含める．
/// 画像のサイズは小さくする
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadImageData {
    image_base64: String,
    noise_info: String,
}

/// 使用する画像をこのサーバに送るための窓口
/// 画像はbase64でエンコードされている．
#[post("/image_upload")]
async fn image_upload(
    request: web::Json<NoisedImage>,
    state: web::Data<AppState>,
) -> impl Responder {
    // 受け取ったのをDBに保存
    let result = NoisedShuttleSharedDb.insert(request.into_inner(), &state.pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    }
}
