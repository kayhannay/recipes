#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod schema;
mod model;
mod request_handler;
mod database;

use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
        .mount("/", routes![request_handler::recipe_list, request_handler::recipe])
        .attach(Template::fairing())
        .attach(database::RecipeDatabase::fairing())
        .launch();
}
