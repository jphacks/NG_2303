-- DROP TABLE IF EXISTS object_detection_data;
DROP TABLE IF EXISTS noised_images;

CREATE TABLE IF NOT EXISTS object_detection_data (
    id     serial    PRIMARY KEY     NOT NULL,
    image_url    TEXT          NOT NULL,
    object_label    TEXT          NOT NULL,
    predicted_label    TEXT          NOT NULL,
    confidence    REAL          NOT NULL,
    forbidden_label    BOOLEAN          NOT NULL,
    -- 本来はboor型にしたいが，sqlite3ではboolean型がないためinteger型で代用
    noise_info     TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS noised_images (
    id     serial    PRIMARY KEY     NOT NULL,
    image_url    TEXT          NOT NULL,
    object_label    TEXT          NOT NULL,
    noise_info     TEXT                NOT NULL,
    forbidden_label    BOOLEAN                NOT NULL
    -- 本来はboor型にしたいが，sqlite3ではboolean型がないためinteger型で代用
);

-- テストデータを入れるクエリ

-- noised_images
INSERT INTO noised_images (image_url, object_label, noise_info, forbidden_label) VALUES
    ('https://storage.googleapis.com/jphacks_ng2303/rabbit_mosaic1.png', 'rabit', 'noise_info', false),
    ('https://storage.googleapis.com/jphacks_ng2303/Cropped_Image_34.png', 'rabit', 'noise_info', false),
    ('https://storage.googleapis.com/jphacks_ng2303/Cropped_Image_35.png', 'rabit', 'noise_info', false),
    ('https://storage.googleapis.com/jphacks_ng2303/Cropped_Image_36.png', 'rabit', 'noise_info', false),
    ('https://storage.googleapis.com/jphacks_ng2303/Cropped_Image_37.png', 'rabit', 'noise_info', false),
    ('https://storage.googleapis.com/jphacks_ng2303/Cropped_Image_39.png', 'rabit', 'noise_info', false),
    ('https://storage.googleapis.com/jphacks_ng2303/Cropped_Image_40.png', 'rabit', 'noise_info', false),
    ('https://storage.googleapis.com/jphacks_ng2303/Cropped_Image_41.png', 'rabit', 'noise_info', false);
