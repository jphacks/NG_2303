use serde::{Serialize, Deserialize};
use anyhow::Result;

trait DataAccumu { 
    fn select(&self, ) -> Result<()>;
    fn insert(&self, ) -> Result<()>;
    fn delete(&self, id:u64 ) -> Result<()>;
}

#[derive(Serialize, Deserialize,  Debug)]
struct FrontendData {
    image_label: String,
    image_base64_vec: Vec<CaptchaImages>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CaptchaImages {
    image_base64: String,
    judge_label: Label,
    noize_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Label {
    ShouldUnchoiced,
    CanChoice,
}


