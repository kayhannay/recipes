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
use rocket_contrib::templates::handlebars;
use rocket_contrib::templates::handlebars::RenderError;
use rocket_contrib::templates::Template;
use std::io::Write;

fn selected_helper(
    h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    // just for example, add error check for unwrap
    let left_value = h
        .param(0)
        .ok_or_else(|| RenderError::new("param 0 not found"))
        .map(|v| v.value())
        .unwrap();
    let right_value = h
        .param(1)
        .ok_or_else(|| RenderError::new("param 1 not found"))
        .map(|v| v.value())
        .unwrap();
    if left_value == right_value {
        out.write("selected")?;
    } else {
        out.write("")?;
    }
    Ok(())
}

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
                controller::recipe::index,
                controller::recipe::recipe_list,
                controller::recipe::recipe_list_by_category,
                controller::recipe::recipe,
                controller::recipe::new_recipe,
                controller::recipe::create_recipe,
                controller::recipe::update_recipe,
                controller::recipe::update_recipe_form,
                controller::recipe::user_new_recipe,
                controller::login::login,
                controller::login::login_user,
                controller::login::login_page,
                controller::config::config,
                controller::config::user_config,
                controller::login::logout,
                controller::user::create_user,
                controller::user::update_user_form,
                controller::user::update_user,
                controller::user::delete_user,
                controller::category::create_category,
                controller::category::update_category_form,
                controller::category::update_category,
                controller::category::delete_category,
            ],
        )
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("selected", Box::new(selected_helper));
        }))
        //.attach(Template::fairing())
        .attach(repository::common::RecipeDatabase::fairing());
    repository::common::run_migrations(
        &*repository::common::RecipeDatabase::get_one(&rocket).unwrap(),
    );
    rocket
}
