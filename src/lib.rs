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
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate log;

pub mod common;
pub mod recipe;
pub mod user;

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
                recipe::controller::recipe_list,
                recipe::controller::recipe,
                user::controller::login,
                user::controller::login_user,
                user::controller::login_page,
                user::controller::config,
                user::controller::user_config,
                user::controller::logout,
                user::controller::create_user
            ],
        )
        .attach(Template::fairing())
        .attach(common::repository::RecipeDatabase::fairing());
    common::repository::run_migrations(
        &*common::repository::RecipeDatabase::get_one(&rocket).unwrap(),
    );
    rocket
}
