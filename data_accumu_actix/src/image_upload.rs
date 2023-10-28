use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/// 使用する画像をこのサーバに送るための窓口
/// 画像はbase64でエンコードされている．

async fn image_upload() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
