CREATE TABLE transactions
(
    id              BIGSERIAL PRIMARY KEY,
    chat_id         BIGINT      NOT NULL,
    category_id     BIGSERIAL REFERENCES categories (id),
    amount          BIGINT      NOT NULL,
    amount_modified BOOLEAN     NOT NULL DEFAULT FALSE,
    description     TEXT,
    currency_code   CHAR(3)     NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL,
    updated_at      TIMESTAMPTZ NOT NULL,
    deleted_at      TIMESTAMPTZ
);