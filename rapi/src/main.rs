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

use chrono::{Date, Datelike, Duration, Local, TimeZone, Utc, Weekday};

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
    // let new_treatment = NewTreatment {
    //     name: String::from("nanika"),
    // };
    // diesel::insert_into(treatment)
    //     .values(&new_treatment)
    //     .execute(&connection)
    //     .unwrap();
    // let treatment_data = treatment
    //     // .limit(5)
    //     .first::<Treatment>(&connection)
    //     .expect("Error loading posts");
    // let actions = Action::belonging_to(&treatment_data)
    //     .load::<Action>(&connection)
    //     .expect("error on load actions");
    let calendar_dates = get_calendar_dates();

    let today = Utc::today();
    let calendar_month = format!("{}年{}月", today.year(), today.month());

    let mut ctx = tera::Context::new();
    ctx.insert("calendar_dates", &calendar_dates);
    ctx.insert("calendar_month", &calendar_month);
    let view = tmpl
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

fn get_calendar_dates() -> Vec<CalendarDate> {
    let today = Utc::today();
    let target_date: Date<Utc> = Utc.ymd(today.year(), today.month(), 1);

    let start_date = get_start_date_in_calendar(target_date);

    let mut dates: Vec<CalendarDate> = vec![];
    for i in 0..42 {
        dates.push(CalendarDate::new(start_date + Duration::days(i)));
    }
    dates
}

fn get_start_date_in_calendar(first_date: Date<Utc>) -> Date<Utc> {
    let minus_days = match first_date.weekday() {
        Weekday::Sun => 0,
        Weekday::Mon => -1,
        Weekday::Tue => -2,
        Weekday::Wed => -3,
        Weekday::Thu => -4,
        Weekday::Fri => -5,
        Weekday::Sat => -6,
    };
    first_date + Duration::days(minus_days)
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

#[derive(Debug, Serialize, PartialEq)]
struct CalendarDate {
    date: u32,
    is_today: bool,
    weekday: i8,
    point: i8,
    is_muted: bool,
}

impl CalendarDate {
    fn new(target: Date<Utc>) -> Self {
        let weekday = match target.weekday() {
            Weekday::Sun => 0,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        };
        let today = Utc::today();
        Self {
            date: target.day(),
            is_today: today == target,
            weekday: weekday,
            point: 0,
            is_muted: today.month() != target.month(),
        }
    }
}

#[test]
fn test_get_start_date_in_calendar() {
    let today = Utc::today();
    let target_date: Date<Utc> = Utc.ymd(today.year(), today.month(), 1);
    let start_date = get_start_date_in_calendar(target_date);
    assert_eq!(start_date, Utc.ymd(2021, 4, 25));
}
#[test]
fn test_get_calendar_dates() {
    let calendar_dates = get_calendar_dates();
    let mut compared_dates = vec![];
    let target = Utc.ymd(2021, 4, 25);
    for i in 0..42 {
        compared_dates.push(CalendarDate::new(target + Duration::days(i)));
    }

    assert_eq!(compared_dates, calendar_dates);
}
