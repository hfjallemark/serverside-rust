#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

pub mod db;
pub mod models;
pub mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::get_posts, routes::get_post])
        .attach(db::Db::fairing())
        .launch();
}
