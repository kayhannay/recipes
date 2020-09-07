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
                controller::recipe::recipe_by_id,
                controller::recipe::recipe_new_form,
                controller::recipe::recipe_new_form_user,
                controller::recipe::recipe_new_user,
                controller::recipe::recipe_update_user,
                controller::recipe::recipe_update_form,
                controller::recipe::recipe_update_form_user,
                controller::recipe::recipe_delete,
                controller::recipe::recipe_delete_user,
                controller::login::login,
                controller::login::login_user,
                controller::login::login_page,
                controller::login::logout,
                controller::config::config,
                controller::config::user_config,
                controller::user::user_create_user,
                controller::user::user_update_form,
                controller::user::user_update_form_user,
                controller::user::user_update,
                controller::user::user_delete,
                controller::user::user_delete_user,
                controller::category::category_create_user,
                controller::category::category_update_form,
                controller::category::category_update_form_user,
                controller::category::category_update_user,
                controller::category::category_delete,
                controller::category::category_delete_user,
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
