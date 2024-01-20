-- Add up migration script here

CREATE TABLE IF NOT EXISTS questions (
    question_uuid uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS answers (
    answer_uuid uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    question_uuid uuid NOT NULL REFERENCES questions (question_uuid) ON DELETE CASCADE,
    content VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);