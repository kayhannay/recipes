use diesel::prelude::*;
use domain::recipe::{NewRecipe, Recipe, RecipeName};
use repository;
use repository::common::RecipeDatabase;
use repository::schema::categories;
use repository::schema::recipes;

pub fn get_recipes(connection: &RecipeDatabase) -> Vec<RecipeName> {
    let mut recipe_list: Vec<RecipeName> = recipes::table
        .select((recipes::id, recipes::name, recipes::category))
        .load::<RecipeName>(&**connection)
        .unwrap();
    recipe_list.sort_by(|a, b| a.name.cmp(&b.name));
    recipe_list
}

pub fn get_recipes_by_category(category: i32, connection: &RecipeDatabase) -> Vec<RecipeName> {
    let mut recipe_list: Vec<RecipeName> = recipes::table
        .select((recipes::id, recipes::name, recipes::category))
        .filter(repository::schema::recipes::category.eq(category))
        .load::<RecipeName>(&**connection)
        .unwrap();
    recipe_list.sort_by(|a, b| a.name.cmp(&b.name));
    recipe_list
}

joinable!(recipes -> categories (category));

pub fn get_recipe(id: i32, connection: &RecipeDatabase) -> Option<Recipe> {
    let recipe = repository::schema::categories::dsl::categories
        .inner_join(repository::schema::recipes::dsl::recipes)
        .select((
            repository::schema::recipes::name,
            repository::schema::recipes::ingredients,
            repository::schema::recipes::preparation,
            repository::schema::categories::name,
            repository::schema::recipes::number_people,
            repository::schema::recipes::experience,
            repository::schema::recipes::created,
            repository::schema::recipes::rights,
            repository::schema::recipes::owner,
            repository::schema::recipes::time_need,
        ))
        .filter(repository::schema::recipes::id.eq(id))
        .load::<Recipe>(&**connection)
        .unwrap()
        .first()?
        .clone();
    Some(recipe)
}

pub fn save_recipe(recipe: &NewRecipe, connection: &RecipeDatabase) -> QueryResult<usize> {
    diesel::insert_into(recipes::table)
        .values(recipe)
        .execute(&**connection)
}
