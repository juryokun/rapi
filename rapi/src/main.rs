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
use diesel::sql_query;
use diesel::sql_types::*;
use diesel::Expression;
use dotenv::dotenv;
use std::env;

use actix_files as fs;
use actix_web::middleware::{errhandlers::ErrorHandlerResponse, errhandlers::ErrorHandlers};
use actix_web::{
    dev, error, get, http, middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use tera::Tera;

use chrono::{Date, Datelike, Duration, Local, NaiveDate, TimeZone, Utc, Weekday};
use std::collections::HashMap;

const WEEKS: i64 = 6;
const DAYS_IN_WEEK: i64 = 7;
const DAYS_IN_CALENDAR: i64 = WEEKS * DAYS_IN_WEEK;

//Sqliteコネクションを作る。
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let calendar_dates = get_calendar_dates();

    let today = Utc::today();
    let calendar_month = format!("{}年{}月", today.year(), today.month());

    let mut ctx = tera::Context::new();
    ctx.insert("calendar_dates", &calendar_dates);
    ctx.insert("calendar_month", &calendar_month);
    ctx.insert("weeks", &WEEKS);
    ctx.insert("days", &DAYS_IN_WEEK);
    let view = tmpl
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

fn get_calendar_dates() -> Vec<CalendarDate> {
    let today = Utc::today();
    let target_date: Date<Utc> = Utc.ymd(today.year(), today.month(), 1);

    let start_date = get_start_date_in_calendar(target_date);
    let treatment_summaries = get_treatment_summaries(start_date);

    let mut dates: Vec<CalendarDate> = vec![];
    for i in 0..DAYS_IN_CALENDAR {
        let target_date = start_date + Duration::days(i);
        dates.push(CalendarDate::new(
            target_date,
            treatment_summaries.get(&target_date.naive_local()),
        ));
    }
    dates
}

fn get_treatment_summaries(start_date: Date<Utc>) -> HashMap<NaiveDate, TreatmentSummary> {
    use schema::treatment_summary::dsl::*;
    let connection = establish_connection();

    let end_date = start_date + Duration::days(DAYS_IN_CALENDAR);
    let summaries = treatment_summary
        .filter(date.between(start_date.naive_local(), end_date.naive_local()))
        .load::<TreatmentSummary>(&connection)
        .unwrap();

    let mut treatment_summaries = HashMap::new();
    for summary in summaries {
        treatment_summaries.insert(summary.date, summary);
    }
    treatment_summaries
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
    today_class: String,
    weekday_class: String,
    max_point: i32,
    mode_point: i32,
    muted_class: String,
}

impl CalendarDate {
    fn new(target: Date<Utc>, summary: Option<&TreatmentSummary>) -> Self {
        let today = Utc::today();
        Self {
            date: target.day(),
            today_class: Self::get_today_class(&target, &today),
            weekday_class: Self::get_weekday_class(target),
            max_point: Self::extract_max_point(summary),
            mode_point: Self::extract_mode_point(summary),
            muted_class: Self::get_muted_class(&target, &today),
        }
    }
    fn get_today_class(target: &Date<Utc>, today: &Date<Utc>) -> String {
        if today == target {
            return "day-today".to_string();
        }
        Self::get_none_class()
    }
    fn get_muted_class(target: &Date<Utc>, today: &Date<Utc>) -> String {
        if today.month() != target.month() {
            return "uk-background-muted".to_string();
        }
        Self::get_none_class()
    }
    fn get_weekday_class(target: Date<Utc>) -> String {
        match target.weekday() {
            Weekday::Sun => "day-sun".to_string(),
            Weekday::Sat => "day-sat".to_string(),
            _ => Self::get_none_class(),
        }
    }
    fn extract_max_point(summary: Option<&TreatmentSummary>) -> i32 {
        match summary {
            Some(treatment_summary) => treatment_summary.max_point.unwrap(),
            None => 0,
        }
    }
    fn extract_mode_point(summary: Option<&TreatmentSummary>) -> i32 {
        match summary {
            Some(treatment_summary) => treatment_summary.mode_point.unwrap(),
            None => 0,
        }
    }
    fn get_none_class() -> String {
        "".to_string()
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
    // for i in 0..42 {
    //     compared_dates.push(CalendarDate::new(target + Duration::days(i)));
    // }

    // assert_eq!(compared_dates, calendar_dates);
}

#[test]
fn test_get_treatment_summaries() {
    let start_date = Utc.ymd(2021, 4, 25);
    let summaries = get_treatment_summaries(start_date);
    let ymd = Utc.ymd(2021, 5, 5).naive_local();
    let compare = TreatmentSummary {
        id: 1,
        treatment_id: 1,
        date: ymd,
        max_point: Some(2),
        mode_point: Some(1),
    };
    assert_eq!(summaries[&ymd], compare);
}
