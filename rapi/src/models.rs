use super::schema::*;
use serde::Serialize;

#[derive(Identifiable, Queryable)]
#[table_name = "treatment"]
pub struct Treatment {
    pub id: i32,
    pub name: String,
}
#[derive(Insertable)]
#[table_name = "treatment"]
pub struct NewTreatment {
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[belongs_to(Treatment)]
#[table_name = "action"]
pub struct Action {
    pub id: i32,
    pub treatment_id: i32,
    pub name: String,
}

#[derive(Identifiable, Serialize, Queryable, Associations, Debug, PartialEq, Clone, Copy)]
#[belongs_to(Treatment)]
#[table_name = "treatment_summary"]
pub struct TreatmentSummary {
    pub id: i32,
    pub treatment_id: i32,
    pub date: chrono::NaiveDate,
    pub max_point: Option<i32>,
    pub mode_point: Option<i32>,
}
