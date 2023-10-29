use crate::LabelAndScore;
use anyhow::Result;
use serde_json::json;

#[derive(Debug, serde::Deserialize)]
struct LabelAnnotation {
    mid: String,
    description: String,
    score: f64,
    topicality: f64,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    label_annotations: Vec<LabelAnnotation>,
}

#[derive(Debug, serde::Deserialize)]
struct ApiResponse {
    responses: Vec<Response>,
}

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

    let json: ApiResponse = res.json().await?;

    let label_and_score = LabelAndScore::new(
        image_url.to_string(),
        json.responses[0].label_annotations[0].description.clone(),
        json.responses[0].label_annotations[0].score as f64,
    );

    Ok(label_and_score)
}
