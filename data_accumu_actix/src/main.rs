//! ideaxtechで作ったソースコードです．Cargo docコマンドを使ってみようということで，一応ドキュメントにしてみました．

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};

mod database;
mod accumu;

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

///エントリーポイントです．
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:5000");

    HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(una)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
