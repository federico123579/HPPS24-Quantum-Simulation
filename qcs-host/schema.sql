CREATE TABLE IF NOT EXISTS `programs` (
    `id` INTEGER,
    `filename` TEXT NOT NULL,
    `text` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE TABLE IF NOT EXISTS `contractions` (
    `id` INTEGER,
    `program_id` INTEGER NOT NULL,
    `span` TEXT NOT NULL,
    `left_id` INTEGER DEFAULT NULL,
    `right_id` INTEGER DEFAULT NULL,
    `kind` TEXT NOT NULL,
    `gate_id` INTEGER DEFAULT NULL,
    PRIMARY KEY(`id`),
    FOREIGN KEY(`program_id`) REFERENCES `programs`(`id`),
    FOREIGN KEY(`left_id`, 'right_id') REFERENCES `contractions`(`id`, `id`),
    CHECK(
        (
            kind = 'C'
            AND left_id IS NOT NULL
            AND right_id IS NOT NULL
            AND gate_id IS NULL
        )
        OR (
            kind = 'G'
            AND left_id IS NULL
            AND right_id IS NULL
            AND gate_id IS NOT NULL
        )
    )
);
CREATE TABLE IF NOT EXISTS `gates` (
    `id` INTEGER,
    `name` TEXT NOT NULL UNIQUE,
    `rank` INTEGER NOT NULL,
    -- 2^(2*rank) numbers stored as a binary sequence (R1: 4, R2: 16, R3: 64, ...)
    -- all complex numbers are stored as two 64-bit floats (real, imag) (little endian)
    `data` BLOB NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE TABLE IF NOT EXISTS `experiments` (
    `id` INTEGER,
    `program_id` INTEGER NOT NULL,
    `input_vector` BLOB NOT NULL,
    `output_vector` BLOB NOT NULL,
    PRIMARY KEY(`id`),
    FOREIGN KEY(`program_id`) REFERENCES `programs`(`id`)
);
