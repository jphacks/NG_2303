CREATE TABLE IF NOT EXISTS object_detection_data (
    id     INTEGER    PRIMARY KEY     NOT NULL,
    image_url    TEXT          NOT NULL,
    object_label    TEXT          NOT NULL,
    predicted_label    TEXT          NOT NULL,
    confidence    REAL          NOT NULL,
    forbidden_label    INTEGER          NOT NULL,
    -- 本来はboor型にしたいが，sqlite3ではboolean型がないためinteger型で代用
    noise_info     TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS noised_images (
    id     INTEGER    PRIMARY KEY     NOT NULL,
    image_url    TEXT          NOT NULL,
    object_label    TEXT          NOT NULL,
    noise_info     TEXT                NOT NULL,
    forbidden_label    INTEGER                NOT NULL
    -- 本来はboor型にしたいが，sqlite3ではboolean型がないためinteger型で代用
);