#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate voskhod_lib;
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use diesel::prelude::*;
use voskhod_lib::*;
use voskhod_lib::models::*;
use rocket_contrib::Template;
use tera::Context;

fn main() {
    rocket::ignite()
        .manage(create_db_pool())
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn index(connection: DbConn) -> Template {
    use schema::users::dsl::*;

    let mut context = Context::new();

    let user_list = users.load::<User>(&*connection).expect("Error loading users");

    context.add("users", &user_list);
    // Where `base` is the name of the template
    Template::render("base", &context)
}
