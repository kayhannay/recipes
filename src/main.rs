#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate chrono;
#[macro_use] extern crate diesel;
extern crate bigdecimal;

mod schema;

use rocket_contrib::templates::Template;
use self::schema::rezepte;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use schema::rezepte::all_columns;

#[database("recipe_db")]
struct RecipeDatabase(diesel::MysqlConnection);

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
struct Recipe {
    id: i32,
    name: String,
    ingredients: String,
    preparation: String,
    experience: Option<String>,
    time_needed: Option<String>,
    number_people: Option<BigDecimal>,
    created: NaiveDateTime,
    owner: Option<BigDecimal>,
    rights: Option<BigDecimal>,
    category: Option<i32>
}

#[derive(Debug, Queryable, Serialize)]
struct RecipeName {
    id: i32,
    name: String,
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
fn recipe_list(connection: RecipeDatabase) -> Template {
    let recipe_list: Vec<RecipeName> = rezepte::table
        .select((rezepte::id, rezepte::name))
        .load::<RecipeName>(&*connection)
        .unwrap();
    let model = RecipeOverviewModel { recipe_names: recipe_list };
    Template::render("index", &model)
}

#[get("/recipe/<id>")]
fn recipe(id: i32, connection: RecipeDatabase) -> Template {
    let mut recipe: Recipe = rezepte::table
        .select(all_columns)
        .filter(rezepte::id.eq(id))
        .load::<Recipe>(&*connection)
        .unwrap().first().unwrap().clone();
    recipe = convert_newline(recipe);
    let model = RecipeModel { recipe };
    Template::render("recipe", &model)
}

fn convert_newline(mut recipe: Recipe) -> Recipe {
    recipe.ingredients = recipe.ingredients.replace("\n", "<br />");
    recipe.preparation = recipe.preparation.replace("\n", "<br />");
    return recipe;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![recipe_list, recipe])
        .attach(Template::fairing())
        .attach(RecipeDatabase::fairing())
        .launch();
}
