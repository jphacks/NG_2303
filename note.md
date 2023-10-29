BeJudgedImages {
    object_label: String, //be_judge_imagesに含まれる画像のラベル 1種類しかないはずなので，ここにも持ってきた
    noized_images: [
        {
            image_url: String, // Amazon S3からのURL
            //image_base64: String,になる可能性もある
            object_label: string,
            noize_info: String, //どのような構造の情報が入るのか決定できないためString
            forbidden_label: bool,   // ユーザが選択した場合，botの可能性を疑うことを示すラベル
        },
        {
            image_url: String,// Amazon S3からのURL
            //image_base64: String,になる可能性もある
            object_label: string,
            noize_info: String, //どのような構造の情報が入るのか決定できないためString
            forbidden_label: bool,   // ユーザが選択した場合，botの可能性を疑うことを示すラベル
        },
        ...
    ]
}

/// dbに保存するデータ 1行あたり
ObjectDetectionData {
    image_url: String,    // Amazon S3からのURL
    object_label: String,      // 物体の本来のラベル
    predicted_label: String, // 物体の誤認識の結果のラベル
    confidence: f64,        // 物体の誤認識の結果の確信度
    forbidden_label: bool,   // ユーザが選択した場合，botの可能性を疑うことを示すラベル
    noise_info: String,      // どのようなノイズがかかっているかの情報
}

