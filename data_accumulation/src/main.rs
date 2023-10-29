use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod accumu;
mod database;
mod image_upload;
mod gcp;

use accumu::NoisedImage;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

use actix_web::middleware::Logger;
use actix_web::{
    error,
    web::{Json, ServiceConfig},
    Result,
};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, FromRow};

use anyhow::anyhow;

// #[post["/postcards"]]
// async fn judge_porker(request: web::Json<Request>) -> impl Responder {
//     match porker::million_porker(&request.useCards, request.num) {
//         Ok((role_count, sum_score, loop_num)) => {
//             porker::debug_judge_role(&role_count, loop_num);
//             HttpResponse::Ok().json(Response::new(sum_score, loop_num, role_count))
//         }
//         Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
//     }
// }

///テスト用の関数です．特に意味はありません．helloを返します．
#[get["/"]]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

///テスト用の関数です．特に意味はありません．401コードを返します．
#[get["/Una"]]
async fn una() -> impl Responder {

    // let noied_image = NoisedImage::new(
    //     "https://s3-ap-northeast-1.amazonaws.com/una-noised-images/una.jpg".to_string(),
    //     "una".to_string(),
    //     "noised".to_string(),
    //     false,
    // );

    // let vec_i = vec![noied_image];

    // let json_string = serde_json::to_string(&vec_i).unwrap();



    HttpResponse::Unauthorized().body("una")
}

/// フロントから送られてきた，ユーザが選択した画像を物体検出に投げて，結果をDBに保存しフロントに返す．
#[post["/judge-captcha"]]
async fn judge_captcha(request: web::Json<Vec<NoisedImage>>) -> impl Responder {
    // GCPかAWSに投げる
    let is_human = true;

    for image_url in request.0.iter() {
        println!("{}", image_url.image_url);
    }

    HttpResponse::Ok().json(is_human)
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

    let frontend_data = crate::database::noised_image::select(&object_label, &state.pool).await;
    // Okならデータを返す Errならbad requestを返す
    match frontend_data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    }
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = web::Data::new(AppState { pool });

    // let a  = aws_config::;
    // let b = aws_sdk_s3::Client::new(&a);
    let secret = if let Some(secret) = secret_store.get("GCP_API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    

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
