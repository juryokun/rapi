#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use models::*;

#[macro_use]
extern crate dotenv;

// models.rsとschema.rsのインポート
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

//Sqliteコネクションを作る。
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn index() -> impl Responder {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    let result = posts
        // .limit(5)
        .first::<Post>(&connection)
        .expect("Error loading posts");

    HttpResponse::Ok().body(result.title)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
