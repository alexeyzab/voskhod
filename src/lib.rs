#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;

extern crate dotenv;

extern crate rand;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate chrono;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use diesel::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;

// Type alias for the connection pool
type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool.")
}

// Tuple-struct wrapper that we can write an implementation of `FromRequest` for
pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn generate_referral_code() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}
