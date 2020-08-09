#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate chrono;

use rocket_contrib::templates::Template;
use rocket_contrib::databases::mysql;
use mysql as my;
use mysql::params;
use chrono::NaiveDateTime;

#[database("recipe_db")]
struct RecipeDatabase(my::Conn);

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

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
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
fn recipe_list(mut connection: RecipeDatabase) -> Template {
    let _recipe_names = get_recipes_names(&mut connection.0);
    let model = RecipeOverviewModel { recipe_names: _recipe_names };
    Template::render("index", &model)
}

#[get("/recipe/<id>")]
fn recipe(id: i32, mut connection: RecipeDatabase) -> Template {
    let _recipe = get_recipe(&mut connection.0, &id);
    let model = RecipeModel {
        recipe: _recipe.unwrap(),
    };

    Template::render("recipe", &model)
}

fn get_recipes_names(connection: &mut my::Conn) -> Vec<RecipeName> {
    let recipes =
        connection.prep_exec("SELECT id,name FROM rezepte", ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, name) = my::from_row(row);
                    RecipeName {
                        id,
                        name
                    }
                }).collect()
            }).unwrap();
    return recipes;
}

fn get_recipe(connection: &mut my::Conn, id: &i32) -> Option<Recipe> {
    let mut vector: Vec<Recipe> =
        connection.prep_exec("SELECT id,category,name,ingredients,preparation,experience,time_need,number_people,created,owner,rights FROM rezepte WHERE id=:id", params! { "id" => id })
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, category, name, ingredients, preparation, experience, time_need, number_people, created, owner, rights) = my::from_row(row);
                    Recipe {
                        id,
                        category,
                        name,
                        ingredients,
                        preparation,
                        experience,
                        time_needed: time_need,
                        number_people,
                        created,
                        owner,
                        rights
                    }
                }).collect()
            }).unwrap();
    let recipe = vector.pop();
    return recipe;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![recipe_list, recipe])
        .attach(Template::fairing())
        .attach(RecipeDatabase::fairing())
        .launch();
}
