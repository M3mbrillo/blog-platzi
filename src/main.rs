
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use tera::Tera;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::Post;
use self::schema::posts::dsl::*;


#[get("/ping")]
async fn ping_pong() -> impl Responder {
    return HttpResponse::Ok().body("Pong!");
}

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {

    let conn = pool.get().expect("Error try get db connection from pool");

    return match web::block(move || {posts.limit(10).load::<Post>(&conn)}).await {
        Ok(data) => { 

            let mut ctx = tera::Context::new();

            ctx.insert("posts", &data.unwrap());

            HttpResponse::Ok()
                .content_type("text/html")
                .body(
                    template_manager.render("index.html", &ctx).unwrap()
                )
        }
        Err(err) => HttpResponse::Ok().body("Error to read data")
    };
}

#[post("/post")]
async fn create_post(pool: web::Data<DbPool>, item: web::Json<models::NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Error get a connection DB from the pool");

    println!("{:?}", item);

    match web::block(move || { Post::create_post(&conn, &item) }).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(err) => HttpResponse::Ok().body("Error to read data")
    }
}

#[get("/tera")]
async fn tera_index(template_manager: web::Data<tera::Tera>) -> impl Responder {

    let ctx = tera::Context::new();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            template_manager.render("index.html", &ctx)
            .unwrap()
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Dont setter DATABASE_URL");

    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(connection).expect("Error build pool connection");

    HttpServer::new(move || {

        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            // endpoints
            .service(index)
            .service(create_post)
            .service(tera_index)
            // Dependencies
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
    })
    .bind(("0.0.0.0", 5700))
    ?.run()
    .await
}
