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

use actix_files as fs;
use actix_web::middleware::{errhandlers::ErrorHandlerResponse, errhandlers::ErrorHandlers};
use actix_web::{
    dev, error, get, http, middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use tera::Tera;

//Sqliteコネクションを作る。
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    use schema::treatment::dsl::*;
    let connection = establish_connection();
    let treatment_data = treatment
        // .limit(5)
        .first::<Treatment>(&connection)
        .expect("Error loading posts");
    let actions = Action::belonging_to(&treatment_data)
        .load::<Action>(&connection)
        .expect("error on load actions");

    let mut ctx = tera::Context::new();
    ctx.insert("actions", &actions);
    let view = tmpl
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

fn not_found<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let new_resp = fs::NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=INFO");
    env_logger::init();

    HttpServer::new(|| {
        let error_handlers = ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, not_found);
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(tera)
            .wrap(error_handlers)
            .wrap(middleware::Logger::default())
            // .service(fs::Files::new("/templates", ".").show_files_listing())
            .service(fs::Files::new(
                "/static",
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("static"),
            ))
            .route("/index", web::get().to(index))
        // .service(web::resource("/").route(web::get().to(index)))
        // .service(fs::Files::new("/", "./templates").route(web::get().to(index)))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
