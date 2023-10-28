CREATE TABLE IF NOT EXISTS object_detection_data (
    id     INTEGER    PRIMARY KEY     NOT NULL,
    label    TEXT          NOT NULL,
    image_recognition_result     TEXT                NOT NULL,
    base64_image     TEXT                NOT NULL
);
