use diesel::prelude::*;
use domain::recipe::{Recipe, RecipeName};
use repository::common::RecipeDatabase;
use repository::schema::rezepte;

pub fn get_recipes(connection: &RecipeDatabase) -> Vec<RecipeName> {
    let recipe_list: Vec<RecipeName> = rezepte::table
        .select((rezepte::id, rezepte::name))
        .load::<RecipeName>(&**connection)
        .unwrap();
    recipe_list
}

pub fn get_recipe(id: i32, connection: &RecipeDatabase) -> Option<Recipe> {
    let recipe = rezepte::table
        .select((
            rezepte::name,
            rezepte::ingredients,
            rezepte::preparation,
            rezepte::category,
            rezepte::number_people,
            rezepte::experience,
            rezepte::created,
            rezepte::rights,
            rezepte::owner,
            rezepte::time_need,
        ))
        .filter(rezepte::id.eq(id))
        .load::<Recipe>(&**connection)
        .unwrap()
        .first()?
        .clone();
    Some(recipe)
}

pub fn save_recipe(recipe: &Recipe, connection: &RecipeDatabase) -> QueryResult<usize> {
    diesel::insert_into(rezepte::table)
        .values(recipe)
        .execute(&**connection)
}
