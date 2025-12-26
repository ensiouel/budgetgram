SELECT *
FROM categories
WHERE chat_id = $1
  AND direction = $2
  AND deleted_at IS NULL
ORDER BY created_at DESC, id DESC;