#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Dont setter DATABASE_URL");

    let conn = PgConnection::establish(&db_url).expect("Error connecting to db");

    use self::models::{NewPost, Post, PostSimplificado};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    // Insert
    // let new_post = NewPost {
    //     title: "My Title",
    //     body: "My Body",
    //     slug: "primer-post"
    // };

    //
    // diesel::insert_into(posts::table)
    //     .values(&new_post)
    //     .get_result::<Post>(&conn)
    //     .expect("Insert into Post fail!");

    // Select * from post
    // let posts_result = posts.load::<Post>(&conn).expect("Error reading Post table");

    // for p in posts_result {
    //     println!("p -> {}", p.title)
    // }

    // Select locos con where
    // let posts_simplificado_result = posts
    //     .order(id.desc())
    //     .select((title, body))
    //     .load::<PostSimplificado>(&conn)
    //     .expect("Error reading Post table");

    // for p in posts_simplificado_result {
    //     println!("{:?}", p)
    // }

    // let posts_result = posts
    //     .filter(self::schema::posts::dsl::slug.like("%segundo%"))
    //     .load::<Post>(&conn).expect("Error reading Post table");

    // for p in posts_result {
    //     println!("{:?}", p)
    // }

    // Update
    let filter_first_post = posts.filter(id.eq(1));
    // let post_update = diesel::update(filter_first_post)
    //     .set((
    //         slug.eq("edited-slug"),
    //         body.eq("edited-body")
    //     )).get_result::<Post>(&conn);

    // println!("{:?}", post_update);

    // Delete
    // diesel::delete(filter_first_post).execute(&conn).expect("Error deleting post");

    // for post in posts.load::<Post>(&conn).expect("Error given all posts")
    // {
    //     println!("{:?}", post);
    // }

    println!("End!");
}
