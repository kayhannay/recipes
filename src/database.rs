use model::{RecipeName, Recipe};
use schema::rezepte;
use diesel::prelude::*;
use schema::rezepte::all_columns;

#[database("recipe_db")]
pub struct RecipeDatabase(diesel::MysqlConnection);

pub fn get_recipes(connection: RecipeDatabase) -> Vec<RecipeName> {
    let recipe_list: Vec<RecipeName> = rezepte::table
        .select((rezepte::id, rezepte::name))
        .load::<RecipeName>(&*connection)
        .unwrap();
    recipe_list
}

pub fn get_recipe(id: i32, connection: RecipeDatabase) -> Recipe {
    let recipe: Recipe = rezepte::table
        .select(all_columns)
        .filter(rezepte::id.eq(id))
        .load::<Recipe>(&*connection)
        .unwrap().first().unwrap().clone();
    recipe
}