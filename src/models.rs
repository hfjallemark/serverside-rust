pub struct Comment {
    pub avatar: String,
    pub content: String,
    pub id: i64,
    pub posted: i64,
    pub user: String,
}

pub struct Post {
    pub avatar: String,
    pub comment_count: i64,
    pub content: String,
    pub id: i64,
    pub posted: i64,
    pub resource_id: String,
    pub resource_type: String,
    pub user: String,
}

#[derive(FromForm)]
pub struct NewPost {
    pub content: String,
    pub resource_id: String,
    pub resource_type: String,
}
