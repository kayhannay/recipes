#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
extern crate dotenv;
extern crate env_logger;
extern crate chrono;

mod schema;
pub mod model;
mod request_handler;
pub mod database;
mod auth;

use rocket_contrib::templates::Template;
use env_logger::Builder;
use chrono::Local;
use log::LevelFilter;
use std::io::Write;

fn init_logging() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S%Z"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

pub fn init_application() -> rocket::Rocket {
    init_logging();
    dotenv::dotenv().ok();
    let rocket = rocket::ignite()
        .mount("/", routes![request_handler::recipe_list, request_handler::recipe, auth::login, auth::login_user, auth::login_page, auth::config, auth::user_config, auth::logout, auth::create_user])
        .attach(Template::fairing())
        .attach(database::RecipeDatabase::fairing());
    database::run_migrations(&*database::RecipeDatabase::get_one(&rocket).unwrap());
    rocket
}
