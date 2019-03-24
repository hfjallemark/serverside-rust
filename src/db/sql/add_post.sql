WITH new_user AS (
	INSERT INTO post (content, created_at, resource_id, resource_type, updated_at, user_id)
	VALUES ($1, now(), $2, $3, now(), (SELECT id FROM "user" LIMIT 1))
	RETURNING *
)
SELECT
  u.avatar,
  (SELECT COUNT(*) FROM comment WHERE post_id = p.id) AS comment_count,
  p.content,
  p.id,
  cast(extract(epoch FROM p.created_at) AS BIGINT) * 1000,
  p.resource_id,
  p.resource_type,
  u.name
FROM
	new_user u
JOIN
	"user" u ON u.id = c.user_id
