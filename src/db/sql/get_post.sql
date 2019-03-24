SELECT
  u.avatar,
  (SELECT COUNT(*) FROM app.comment WHERE post_id = p.id) AS comment_count,
  p.content,
  p.id,
  cast(extract(epoch FROM p.created_at) AS BIGINT) * 1000,
  p.resource_id,
  p.resource_type,
  u.name
FROM
  app.post p
JOIN
  app.user u ON u.id = p.user_id
WHERE
  p.id = $1
