use reqwest::Error;

use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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

#[tokio::main]
async fn main() -> Result<()> {
    //     let response = reqwest::get("https://www.google.com").await?;
    //     println!("Status: {}", response.status());
    //     println!("Headers:\n{:?}", response.headers());
    //     println!("Body:\n{}", response.text().await?);

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    Ok(())
}
