use rusoto_core::Region;
use rusoto_rekognition::{DetectLabelsRequest, Image, Rekognition, RekognitionClient, S3Object, Label};
use anyhow::Result;

pub fn get_highest_confidence_label(labels: &[rusoto_rekognition::Label]) -> Option<&rusoto_rekognition::Label> {
    // ラベルリストが空の場合はNoneを返す
    if labels.is_empty() {
        return None;
    }

    // 最初のラベルを初期最高信頼度ラベルとして設定
    let mut highest_confidence_label = &labels[0];

    // ラベルリストを走査して最高信頼度のラベルを見つける
    for label in labels.iter().skip(1) {
        if let Some(confidence) = label.confidence {
            if confidence > highest_confidence_label.confidence.unwrap_or(0.0) {
                highest_confidence_label = label;
            }
        }
    }

    // 最高信頼度のラベルを返す
    Some(highest_confidence_label)
}


pub async fn detect_labels() -> Result<(Label)> {
    // AWSリージョン
    let region = Region::ApNortheast1; // "ap-northeast-1"に対応するリージョン

    // AWS認証設定
    let rekognition_client = RekognitionClient::new(region);

    // S3オブジェクトの設定
    let s3_object = S3Object {
        bucket: Some("jphacks".to_string()),
        name: Some("dog_ai1.png".to_string()),
        version: None,
    };

    // 画像解析の設定
    let rekognition_image = Image {
        s3_object: Some(s3_object),
        ..Default::default()
    };

    // 画像解析リクエストの設定
    let request = DetectLabelsRequest {
        image: rekognition_image,
        max_labels: Some(10),    // 何個までラベルを取得するかの設定
        min_confidence: Some(85.0),  // 信頼度の閾値
        ..Default::default()
    };

    // 画像解析の実行
    let response = rekognition_client.detect_labels(request).await?;
    
    // // 結果の表示
    // if let Some(highest_confidence_label) = get_highest_confidence_label(response.labels.as_deref().unwrap_or(&[])) {
    //     // println!("最も信頼度が高いラベル: {}, 信頼度: {}%", highest_confidence_label.name.as_ref().unwrap(), highest_confidence_label.confidence.unwrap());
    //     println!("name: {}", highest_confidence_label.name.as_ref().unwrap())
    // } else {
    //     println!("ラベルがありません。");
    // }
        let a = get_highest_confidence_label(response.labels.as_deref().unwrap()).unwrap().clone();

        Ok(a)
        }