#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use rocket_contrib::Template;
use tera::Context;

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.add("my_message", "Hello, Template World!");
    // Where `base` is the name of our template
    Template::render("base", &context)
}
