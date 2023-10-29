use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod accumu;
mod aws_s3;
mod database;
mod image_upload;

use accumu::NoizedImage;
use sqlx::SqlitePool;

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
    let a = 1;
    if a == 1 {
        HttpResponse::Ok().body("hello")
    } else {
        HttpResponse::Unauthorized().body("401 Unauthrized")
    }
}

/// フロントから送られてきた，ユーザが選択した画像を物体検出に投げて，結果をDBに保存しフロントに返す．
#[post["/judge-captcha"]]
async fn judge_captcha(_request: web::Json<NoizedImage>) -> impl Responder {
    // match porker::million_porker(&request.useCards, request.num) {
    //     Ok((role_count, sum_score, loop_num)) => {
    //         porker::debug_judge_role(&role_count, loop_num);
    //         HttpResponse::Ok().json(Response::new(sum_score, loop_num, role_count))
    //     }
    //     Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    // }

    //
    let is_human = true;

    HttpResponse::Ok().json(is_human)
}

/// フロントに表示するノイズかけた画像を送る
/// 画像はbase64でエンコードされている．
/// ラベル，
#[get["/get-capthcha-images"]]
async fn get_captha_images() -> impl Responder {
    let frontend_data = NoizedImage::new_dammy();
    HttpResponse::Ok().json(frontend_data)
}

///エントリーポイントです．
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:5000");

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let state = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .service(get_index)
            .service(una)
            .service(judge_captcha)
            .service(judge_captcha)
            .app_data(state.clone())
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
