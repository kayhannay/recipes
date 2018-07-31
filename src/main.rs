#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;
extern crate chrono;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::io::Error;
use std::collections::HashMap;
use mysql as my;
//use mysql::chrono::DateTime;
//use mysql::chrono::Utc;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq)]
struct Recipe {
    id: i32,
    category: i32,
    name: String,
    ingredients: String,
    preparation: String,
    experience: String,
    time_needed: String,
    number_people: String,
    created: NaiveDateTime,
    owner: String,
    rights: String,
}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>
}

#[derive(Serialize)]
struct RecipeOverviewModel {
    recipe_names: Vec<String>
}

#[get("/")]
fn index() -> Template {
    let pool = init_db();
    let recipes = get_recipes(pool);
    let model = RecipeOverviewModel {
        recipe_names: recipes.iter().map(|r|r.name.to_string()).collect()
    };
    Template::render("index", &model)
}

#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    let context = TemplateContext {
        name: name,
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect()
    };

    Template::render("hello", &context)
}

fn init_db() -> my::Pool {
    let pool = my::Pool::new("mysql://rezepte:my-secret@127.0.0.1:3306/rezepte").unwrap();
    return pool;
}

fn get_recipes(pool: mysql::Pool) -> Vec<Recipe> {
    let recipes: Vec<Recipe> =
        pool.prep_exec("SELECT * from rezepte", ())
            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
                // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `Payment`
                result.map(|x| x.unwrap()).map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow your schema
                    let (id, name, ingredients, preparation, experience, time_needed, number_people, created, owner, rights, category) = my::from_row(row);
                    Recipe {
                        id: id,
                        category: category,
                        name: name,
                        ingredients: ingredients,
                        preparation: preparation,
                        experience: experience,
                        time_needed: time_needed,
                        number_people: number_people,
                        created: created,
                        owner: owner,
                        rights: rights,
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
            }).unwrap(); // Unwrap `Vec<Payment>`
    return recipes;
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).attach(Template::fairing()).launch();
}

