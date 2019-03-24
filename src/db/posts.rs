use crate::db::Db;
use crate::models::{NewPost, Post};
use postgres::rows::Row;
use postgres::Result;

impl Db {
    pub fn add_post(self: &Db, new: &NewPost, _: i64) -> Result<Post> {
        self.query_one(
            sql!("add_post"),
            &[&new.content, &new.resource_id, &new.resource_type],
            Post::from_row,
        )
    }

    pub fn get_post(self: &Db, id: i64) -> Result<Post> {
        self.query_one(sql!("get_post"), &[&id], Post::from_row)
    }

    pub fn get_posts(self: &Db) -> Result<Vec<Post>> {
        self.query_list(sql!("get_posts"), &[], Post::from_row)
    }
}

impl Post {
    fn from_row(row: Row) -> Post {
        Post {
            avatar: row.get(0),
            comment_count: row.get(1),
            content: row.get(2),
            id: row.get(3),
            posted: row.get(4),
            resource_id: row.get(5),
            resource_type: row.get(6),
            user: row.get(7),
        }
    }
}
