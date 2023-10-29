use crate::LabelAndScore;
use anyhow::Result;
use serde_json::{json, Value};


/// visionAPIにリクエストを投げて，物体検出する 結果はラベルとスコアの組で返す
pub async fn object_detect(image_url: &str, api_key: &str) -> Result<LabelAndScore> {
    let access_url = format!(
        "https://vision.googleapis.com/v1/images:annotate?key={}",
        api_key
    );

    let body = json!({
        "requests": [
            {
                "image": {
                    "source": {
                        "imageUri": image_url
                    }
                },
                "features": [
                    {
                        "type":"LABEL_DETECTION",
                        "maxResults":1
                    }
                ]
            }
        ]
    });

    let client = reqwest::Client::new();
    let res = client
        .post(&access_url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    log::info!("res: {:?}", res);

    // 構造を定義するのが大変なので，Value型を使う
    let value :Value = res.json().await?;

    let label = value[0]["labelAnnotations"][0]["description"].to_string();
    let score = value[0]["labelAnnotations"][0]["score"].to_string().parse()?;

    let label_and_score = LabelAndScore::new(
        image_url.to_string(),
        label,
        score
    );

    Ok(label_and_score)
}
