UPDATE transactions
SET category_id     = $3,
    amount          = $4,
    amount_modified = $5,
    description     = $6,
    currency_code   = $7,
    updated_at      = $8
WHERE id = $1
  AND chat_id = $2
  AND deleted_at IS NULL;