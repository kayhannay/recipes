#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate bigdecimal;
extern crate chrono;
extern crate crypto;
extern crate dotenv;
extern crate env_logger;
extern crate log;

pub mod controller;
pub mod domain;
pub mod repository;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use rocket_contrib::templates::Template;
use std::io::Write;

pub fn init_logging() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
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
    dotenv::dotenv().ok();
    let rocket = rocket::ignite()
        .mount(
            "/",
            routes![
                controller::recipe::recipe_list,
                controller::recipe::recipe,
                controller::recipe::new_recipe,
                controller::recipe::create_recipe,
                controller::recipe::user_new_recipe,
                controller::login::login,
                controller::login::login_user,
                controller::login::login_page,
                controller::config::config,
                controller::config::user_config,
                controller::login::logout,
                controller::user::create_user
            ],
        )
        .attach(Template::fairing())
        .attach(repository::common::RecipeDatabase::fairing());
    repository::common::run_migrations(
        &*repository::common::RecipeDatabase::get_one(&rocket).unwrap(),
    );
    rocket
}
