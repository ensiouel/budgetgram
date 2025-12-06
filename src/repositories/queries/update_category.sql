UPDATE categories
SET name          = $3,
    label         = $4,
    direction     = $5,
    is_regular    = $6,
    target_amount = $7,
    updated_at    = $8
WHERE id = $1
  AND chat_id = $2
  AND deleted_at IS NULL;