CREATE TABLE categories
(
    id            BIGSERIAL PRIMARY KEY,
    chat_id       BIGINT      NOT NULL,
    name          TEXT        NOT NULL,
    label         TEXT        NOT NULL,
    direction     INTEGER     NOT NULL,
    is_regular    BOOLEAN     NOT NULL DEFAULT true,
    target_amount BIGINT      NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL,
    updated_at    TIMESTAMPTZ NOT NULL,
    deleted_at    TIMESTAMPTZ,
    UNIQUE (chat_id, name, label)
);