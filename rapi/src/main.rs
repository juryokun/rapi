#[macro_use]
extern crate diesel;

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

use actix_web::{dev, web, http, App, HttpServer, HttpResponse, Error, error};
use actix_web::middleware::{errhandlers::ErrorHandlers, errhandlers::ErrorHandlerResponse};
use actix_files as fs;
use tera::Tera;

//Sqliteコネクションを作る。
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
//     use schema::posts::dsl::*;
//     let connection = establish_connection();
//     let result = posts
//         // .limit(5)
//         .first::<Post>(&connection)
//         .expect("Error loading posts");

//     let mut ctx = tera::Context::new();
//     ctx.insert("name", &result.title);
//     let view =
//         tmpl.render("index.html", &ctx)
//             .map_err(|e| error::ErrorInternalServerError(e))?;

//     Ok(HttpResponse::Ok().content_type("text/html").body(view))
// }
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/plain").body("Hello World"))
}

fn not_found<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let new_resp = fs::NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new( || {
        App::new()
            .service(web::resource("/").to(index))
    })
    // HttpServer::new( || {

    //     let error_handlers = ErrorHandlers::new()
    //         .handler(http::StatusCode::NOT_FOUND, not_found);
    //     // let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    //     let tera = Tera::new("templates/**/*").unwrap();


    //     App::new()
    //         .data(tera)
    //         .wrap(error_handlers)
    //         .service(web::resource("/").route(web::get().to(index)))
    // })
    .bind("localhost:3000")?
    .run()
    .await
}