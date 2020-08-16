use common::repository::RecipeDatabase;
use common::schema::rezepte;
use common::schema::rezepte::all_columns;
use diesel::prelude::*;
use recipe::model::{Recipe, RecipeName};

pub fn get_recipes(connection: RecipeDatabase) -> Vec<RecipeName> {
    let recipe_list: Vec<RecipeName> = rezepte::table
        .select((rezepte::id, rezepte::name))
        .load::<RecipeName>(&*connection)
        .unwrap();
    recipe_list
}

pub fn get_recipe(id: i32, connection: RecipeDatabase) -> Option<Recipe> {
    let recipe = rezepte::table
        .select(all_columns)
        .filter(rezepte::id.eq(id))
        .load::<Recipe>(&*connection)
        .unwrap()
        .first()?
        .clone();
    Some(recipe)
}

pub fn save_recipe(recipe: &Recipe, connection: RecipeDatabase) {
    diesel::insert_into(rezepte::table)
        .values(recipe)
        .execute(&*connection)
        .expect("Error saving receipt");
}
