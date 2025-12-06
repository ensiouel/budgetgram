SELECT *
FROM categories
WHERE id = $1
  AND chat_id = $2
  AND deleted_at IS NULL;