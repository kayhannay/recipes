#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod schema;
pub mod model;
mod request_handler;
pub mod database;

use rocket_contrib::templates::Template;

pub fn init_application() -> rocket::Rocket {
    let rocket = rocket::ignite()
        .mount("/", routes![request_handler::recipe_list, request_handler::recipe])
        .attach(Template::fairing())
        .attach(database::RecipeDatabase::fairing());
    database::run_migrations(&*database::RecipeDatabase::get_one(&rocket).unwrap());
    rocket
}
