use std::{env, path::{Path, PathBuf}, fs::File, io::Write, time::Duration};

use bytes::Bytes;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use aws_sdk_s3::{operation::{
    copy_object::{CopyObjectError, CopyObjectOutput},
    create_bucket::{CreateBucketError, CreateBucketOutput},
    get_object::{GetObjectError, GetObjectOutput},
    list_objects_v2::ListObjectsV2Output,
    put_object::{PutObjectError, PutObjectOutput},
}, presigning::PresigningConfig};
use aws_sdk_s3::types::{
    BucketLocationConstraint, CreateBucketConfiguration, Delete, ObjectIdentifier,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{error::SdkError, primitives::ByteStream, Client};
use serde::{Deserialize, Serialize};
use tracing::log::trace;
use anyhow::Result;



#[derive(Debug)]
struct Opt {
    bucket: String,
    object: String,
    destination: PathBuf,
}

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(Path::new(file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
}



// // snippet-start:[s3.rust.get_object]
// /// S3ストレージからオブジェクトを取得し，base64文字列にエンコードされた画像を返す
// async fn get_object(client: Client, opt: Opt) -> Result<usize, anyhow::Error> {
//     trace!("bucket:      {}", opt.bucket);
//     trace!("object:      {}", opt.object);
//     trace!("destination: {}", opt.destination.display());

//     let mut file = File::create(opt.destination.clone())?;

//     let mut object = client
//         .get_object()
//         .bucket(opt.bucket)
//         .key(opt.object)
//         .send()
//         .await?;

//     // let mut byte_count = 0_usize;
//     // while let Some(bytes) = object.body.try_next().await? {
//     //     let bytes = file.write(&bytes)?;
//     //     byte_count += bytes;
//     //     trace!("Intermediate write of {bytes}");
//     // }



//     Ok(byte_count)
// }
// snippet-end:[s3.rust.get_object]

pub async fn list_objects(client: &Client, bucket_name: &str) -> Result<()> {
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
    println!("Objects in bucket:");
    for obj in objects.contents().unwrap_or_default() {
        println!("{:?}", obj.key().unwrap());
    }

    Ok(())
}
async fn get_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<()> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(())
}

async fn put_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<()> {
    let expires_in = Duration::from_secs(expires_in);

    let presigned_request = client
        .put_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(())
}