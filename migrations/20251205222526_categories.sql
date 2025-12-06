CREATE TABLE categories
(
    id            BIGSERIAL PRIMARY KEY,
    chat_id       BIGINT                   NOT NULL,
    name          TEXT                     NOT NULL,
    label         TEXT                     NOT NULL,
    direction     INTEGER                  NOT NULL,
    is_regular    BOOLEAN                  NOT NULL,
    target_amount BIGINT,
    created_at    TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at    TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at    TIMESTAMP WITH TIME ZONE,
    UNIQUE (chat_id, name, direction)
);
