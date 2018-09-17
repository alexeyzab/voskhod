#![allow(proc_macro_derive_resolution_fallback)]
// Bringing the schema into scope
use schema::users;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub referral_code: String,
    pub referrer_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub referral_code: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
