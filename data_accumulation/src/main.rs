use actix_web::{get, post, web, HttpResponse, Responder, Result};

mod accumu;
mod database;
mod gcp;
mod image_upload;

use accumu::NoisedImage;
use serde::{Serialize, Deserialize};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

use actix_web::web::ServiceConfig;

use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

use anyhow::{anyhow};

use crate::accumu::BeJudgeImages;
use crate::database::{noised_image::NoisedShuttleSharedDb, object_detected::ObjectShuttleSharedDb};
use crate::accumu::{NoisedImageStore, ObjectDetectionDataStore};


/// ラベルとスコアの組
#[derive(Debug, Serialize, Deserialize)]
pub struct LabelAndScore {
    pub image_url: String,
    pub label: String,
    pub score: f64,
}

impl LabelAndScore {
    pub fn new(image_url: String, label: String, score: f64) -> Self { Self { image_url, label, score } }
}

#[async_trait::async_trait]
pub trait objectDetector {
    /// ラベルのスコアの組を返す
    async fn object_detect(&self, image_url: &str, api_key: &str,) -> Result<LabelAndScore>;
}
///テスト用の関数です．特に意味はありません．helloを返します．
#[get["/"]]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

///テスト用の関数です．特に意味はありません．
#[get["/Una"]]
async fn una(state: web::Data<AppState>) -> impl Responder {
    // let noied_image = NoisedImage::new(
    //     "https://s3-ap-northeast-1.amazonaws.com/una-noised-images/una.jpg".to_string(),
    //     "una".to_string(),
    //     "noised".to_string(),
    //     false,
    // );

    // let vec_i = vec![noied_image];

    // let json_string = serde_json::to_string(&vec_i).unwrap();


    let sercrt = state.secret.clone();
    let image_path = "gs://cloud-samples-data/vision/demo-img.jpg";

    let a = crate::gcp::object_detect(&sercrt, image_path).await;

    match a {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    }
}

/// フロントから送られてきた，ユーザが選択した画像を物体検出に投げて，結果をDBに保存しフロントに返す．
#[post["/judge-captcha"]]
async fn judge_captcha(request: web::Json<BeJudgeImages>, state: web::Data<AppState>) -> Result<impl Responder> {
    // GCPかAWSに投げる

    let images_data = request.noized_images.iter();

    let sercrt = state.secret.clone();


    let mut objects_detected = Vec::new();
    for image in images_data {
        let result = crate::gcp::object_detect(&image.image_url, &sercrt).await.ok();
        if let Some(label_and_score) = &result {
            // dbに保存
            let object_detected = crate::accumu::ObjectDetectionData::new(
                image.image_url.clone(),
                image.object_label.clone(),
                label_and_score.label.clone(),
                label_and_score.score,
                image.forbidden_label,
                image.noise_info.clone(),
            );

            let r  = ObjectShuttleSharedDb.insert(object_detected, &state.pool).await;
            match r {
                Err(e) => log::error!("error: {}", e),
                _ => {}
            }
        }
        objects_detected.push(result);
    }

    // 結果をもとに処理したis_humanを返す処理も検討していた
    // let is_human = true;

    // image_urlとpredicted_labelとスコアのベクタを返す

    Ok(HttpResponse::Ok().json(objects_detected))
}

/// フロントに表示するノイズかけた画像を送る
/// 画像はbase64でエンコードされている．
/// ラベル，
#[get["/get-capthcha-images/{object_label}"]]
async fn get_captha_images(
    state: web::Data<AppState>,
    object_label: web::Path<String>,
) -> impl Responder {
    // let pool = pool.pool;
    // let object_label = object_label.clone();
    // HttpResponse::Ok().body(object_label)

    let frontend_data = NoisedShuttleSharedDb.select(&object_label, &state.pool).await;
    // Okならデータを返す Errならbad requestを返す
    match frontend_data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    }
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    secret: String,
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // let a  = aws_config::;
    // let b = aws_sdk_s3::Client::new(&a);
    let secret = if let Some(secret) = secret_store.get("GCP_API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    let state = web::Data::new(AppState { pool, secret });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(get_index)
                .service(una)
                .service(judge_captcha)
                // .service(judge_captcha)
                .service(get_captha_images)
                .service(image_upload::image_upload)
                // .app_data(state.clone())
                .app_data(state),
        );
    };

    Ok(config.into())
}
// ///エントリーポイントです．
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
//     log::info!("starting HTTP server at http://localhost:5000");

//     let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
//         .await
//         .unwrap();

//     let state = web::Data::new(AppState { pool });

//     HttpServer::new(move || {
//         App::new()
//             .service(get_index)
//             .service(una)
//             .service(judge_captcha)
//             .service(judge_captcha)
//             .service(get_captha_images)
//             .app_data(state.clone())
//     })
//     .bind(("127.0.0.1", 5001))?
//     .run()
//     .await
// }
