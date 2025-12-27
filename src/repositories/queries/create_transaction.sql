INSERT INTO transactions (chat_id, category_id, amount, description, currency_code, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING *;