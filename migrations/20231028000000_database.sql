CREATE TABLE IF NOT EXISTS acumulate_data (
    id     TEXT    PRIMARY KEY     NOT NULL,
    label    TEXT          NOT NULL,
    image_recognition_result     TEXT                NOT NULL,
    base64_image     TEXT                NOT NULL,
    created_at     TIMESTAMP     NOT NULL
);
