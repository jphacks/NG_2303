//使うクレート
extern crate rusoto_core;
extern crate rusoto_rekognition;
extern crate tokio;

//使うdependenciesの設定
use rusoto_core::Region;
use rusoto_rekognition::{DetectLabelsRequest, Image, Rekognition, RekognitionClient, S3Object};
use anyhow::Result;

use rusqlite::{params, Connection};

//データベースを元に構造体を作成
struct NoisedImages {
    id: i32,
    image_url: String,
    object_label: String,
    noise_info: String,
    forbidden_label: i32,
}

//AWSのRekognitionで得たラベルを走査される関数
fn get_highest_confidence_label(labels: &[rusoto_rekognition::Label]) -> Option<&rusoto_rekognition::Label> {
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

//AWS周りを動かす関数
async fn detect_labels(filename: &str, rekognition_client: &RekognitionClient, conn: &Connection) -> Result<()> {

    // S3オブジェクトの設定
    let s3_object = S3Object {
        bucket: Some("使っているS3バケット".to_string()), // S3バケットの名前
        name: Some(filename.to_string()), // S3バケット直下にある画像の名前
        version: None, // S3のバージョン(今回は指定なし)
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
    // 結果の表示
    if let Some(highest_confidence_label) = get_highest_confidence_label(response.labels.as_deref().unwrap_or(&[])) {
        
        //走査の結果表示
        println!("name: {}", highest_confidence_label.name.as_ref().unwrap())
    } else {
        println!("ラベルがありません。{}", filename);
    }

    //データベースにアクセスしてデータを保存する。

    if let Some(highest_confidence_label) = get_highest_confidence_label(response.labels.as_deref().unwrap_or(&[])) {
        let new_image_data = NoisedImages {
            id: 0, // 自動増分のため0を指定
            image_url: filename.to_string(), // 画像のURLやファイルパス
            object_label: highest_confidence_label.name.as_ref().unwrap().to_string(),
            noise_info: "some noise info".to_string(), // ノイズ情報（必要に応じて設定）
            forbidden_label: 0, // 禁止ラベルのID（必要に応じて設定）
        };
        conn.execute(
            "INSERT INTO noised_images (image_url, object_label, noise_info, forbidden_label)
            VALUES (?1, ?2, ?3, ?4)",
            params![new_image_data.image_url, new_image_data.object_label, new_image_data.noise_info,
                    new_image_data.forbidden_label]
        )?;
        println!("データをデータベースに保存しました。{}", filename);
    } else {
        println!("ラベルがありません。{}", filename);
    }

    Ok(())
}

async fn process_iamges() -> Result<()> {
    // AWSリージョン
    let region = Region::ApNortheast1; // "ap-northeast-1"に対応するリージョン

    // AWS認証設定
    let rekognition_client = RekognitionClient::new(region);

    // データベースの接続
    let conn = Connection::open("noised_db.db3")?;

    for i in 1..=8 {
        let filename = format!("image{}.png", i);
        detect_labels(&filename, &rekognition_client, &conn).await?;
    }
    Ok(())
}

//非同期処理の実行
#[tokio::main]
async fn main() {
    if let Err(err) = process_iamges().await {
        eprintln!("Error: {:?}", err);
    }
}
