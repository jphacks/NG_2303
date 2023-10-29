use anyhow::Result;
use sqlx::any::AnyRow;
use sqlx::Database;
use sqlx::PgPool;

use crate::accumu::DataAccumu;
use crate::accumu::DataStore;
use crate::accumu::NoisedImage;
use crate::accumu::ObjectDetectionData;

pub struct DbQuery;
pub mod noised_image;
pub mod object_detected;
