INSERT INTO categories (chat_id, name, label, direction, is_regular, target_amount, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING *;