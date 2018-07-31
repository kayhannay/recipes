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

#[derive(Debug, PartialEq, Eq, Serialize)]
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

#[derive(Debug, PartialEq, Eq, Serialize)]
struct RecipeName {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>,
}

#[derive(Serialize)]
struct RecipeOverviewModel {
    recipe_names: Vec<RecipeName>,
}

#[derive(Serialize)]
struct RecipeModel {
    recipe: Recipe,
}

#[get("/")]
fn index() -> Template {
    let pool = init_db();
    let _recipe_names = get_recipes_names(pool);
    let model = RecipeOverviewModel { recipe_names: _recipe_names };
    Template::render("index", &model)
}

#[get("/recipe/<id>")]
fn hello(id: i32) -> Template {
    let pool = init_db();
    let _recipe = get_recipe(pool, id);
    let model = RecipeModel {
        recipe: _recipe.unwrap(),
    };

    Template::render("recipe", &model)
}

fn init_db() -> my::Pool {
    let pool = my::Pool::new("mysql://rezepte:my-secret@127.0.0.1:3306/rezepte").unwrap();
    return pool;
}

fn get_recipes_names(pool: mysql::Pool) -> Vec<RecipeName> {
    let recipes: Vec<RecipeName> =
        pool.prep_exec("SELECT id,name FROM rezepte", ())
            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
                // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `Payment`
                result.map(|x| x.unwrap()).map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow your schema
                    let (id, name) = my::from_row(row);
                    RecipeName {
                        id: id,
                        name: name,
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
            }).unwrap(); // Unwrap `Vec<Payment>`
    return recipes;
}

fn get_recipe(pool: mysql::Pool, id: i32) -> Option<Recipe> {
    let mut stmt = pool.prepare("SELECT * FROM rezepte WHERE id=?").unwrap();
    let mut recipe = None;
    for row in stmt.execute((id,)).unwrap() {
        let mut row = row.unwrap();
        let ingredients: String = row.take("ingredients").unwrap();
        let preparation: String = row.take("preparation").unwrap();
        recipe = Some(Recipe {
            id: row.take("id").unwrap(),
            category: row.take("category").unwrap(),
            name: row.take("name").unwrap(),
            ingredients: ingredients.replace("\n", "<br />"),
            preparation: preparation.replace("\n", "<br />"),
            //preparation: row.take("preparation").unwrap(),
            experience: row.take("experience").unwrap(),
            time_needed: row.take("time_needed").unwrap_or("".to_string()),
            //time_needed: "".to_string(),
            number_people: row.take("number_people").unwrap(),
            created: row.take("created").unwrap(),
            owner: row.take("owner").unwrap(),
            rights: row.take("rights").unwrap(),
        })
    }
    return recipe;
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

