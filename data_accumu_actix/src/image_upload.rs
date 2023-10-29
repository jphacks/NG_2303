use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::types::{
    BucketLocationConstraint, CreateBucketConfiguration, Delete, ObjectIdentifier,
};
use aws_sdk_s3::{error::SdkError, primitives::ByteStream, Client};
use aws_sdk_s3::{
    operation::{
        copy_object::{CopyObjectError, CopyObjectOutput},
        create_bucket::{CreateBucketError, CreateBucketOutput},
        get_object::{GetObjectError, GetObjectOutput},
        list_objects_v2::ListObjectsV2Output,
        put_object::{PutObjectError, PutObjectOutput},
    },
    presigning::PresigningConfig,
};
use serde::{Deserialize, Serialize};
use tracing::log::trace;

use crate::accumu::NoizedImage;

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
async fn image_upload(request: web::Json<NoizedImage>) -> impl Responder {
    // 受け取ったノイズ加工済みデータをAmazon S3に保存する

    HttpResponse::Ok().body("hello")
}
