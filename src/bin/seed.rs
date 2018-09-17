extern crate voskhod_lib;
extern crate chrono;
extern crate diesel;

#[macro_use] extern crate fake;

use chrono::prelude::{Utc};
use diesel::prelude::*;
use voskhod_lib::*;
use voskhod_lib::models::*;

fn main() {
    use schema::users::dsl::*;

    let connection = create_db_pool().get().unwrap();

    diesel::delete(users).execute(&*connection).expect("Error deleting users");

    fn generate_user_info() -> NewUser {
        NewUser {
            email: fake!(Internet.free_email),
            referral_code: generate_referral_code(),
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }

    let new_user_list: Vec<NewUser> = (0..10)
        .map ( |_| generate_user_info() )
        .collect();

    diesel::insert_into(users)
        .values(&new_user_list)
        .get_results::<User>(&*connection)
        .expect("Error inserting users");
}
