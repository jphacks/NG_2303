use serde_json::json;

// visionAPIにリクエストを投げて，物体検出する
pub async fn ocject_detect(api_key: &str, image_path:&str ) -> anyhow::Result<()>{
    let access_url = format!("POST https://vision.googleapis.com/v1/images:annotate?key={}", api_key);

    let body = json!({
        "requests": [
            {
                "image": {
                    "source": {
                        "imageUri": image_path
                    }
                },
                "features": [
                    {
                        "type":"LABEL_DETECTION",
                        "maxResults":10
                    }
                ]
            }
        ]
    });

    let client = reqwest::Client::new();
    let res = client.post(&access_url)
        .json(&body)
        .send()
        .await?;

    log::info!("res: {:?}", res);

    Ok(())
}