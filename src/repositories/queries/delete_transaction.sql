UPDATE transactions
SET deleted_at = $3
WHERE id = $1
  AND chat_id = $2
  AND deleted_at IS NULL;