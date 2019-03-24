use crate::db::Db;
use crate::models::{NewPost, Post};
use askama::Template;
use postgres::Result;
use rocket::request::Form;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostsTemplate {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    post: Post,
}

#[get("/")]
pub fn get_posts(db: Db) -> Result<PostsTemplate> {
    db.get_posts().map(|posts| PostsTemplate { posts: posts })
}

#[get("/<id>")]
pub fn get_post(id: i64, db: Db) -> Result<PostTemplate> {
    db.get_post(id).map(|post| PostTemplate { post: post })
}

#[post("/", data = "<new_post>")]
pub fn add_post(db: Db, new_post: Form<NewPost>) -> Result<PostTemplate> {
    db.add_post(&new_post, 55)
        .map(|post| PostTemplate { post: post })
}
