use diesel::prelude::*;
use domain::recipe::{NewRecipe, Recipe, RecipeName};
use repository;
use repository::common::RecipeDatabase;
use repository::schema::categories;
use repository::schema::rezepte;

pub fn get_recipes(connection: &RecipeDatabase) -> Vec<RecipeName> {
    let recipe_list: Vec<RecipeName> = rezepte::table
        .select((rezepte::id, rezepte::name, rezepte::category))
        .load::<RecipeName>(&**connection)
        .unwrap();
    recipe_list
}

pub fn get_recipes_by_category(category: i32, connection: &RecipeDatabase) -> Vec<RecipeName> {
    let recipe_list: Vec<RecipeName> = rezepte::table
        .select((rezepte::id, rezepte::name, rezepte::category))
        .filter(repository::schema::rezepte::category.eq(category))
        .load::<RecipeName>(&**connection)
        .unwrap();
    recipe_list
}

joinable!(rezepte -> categories (category));

pub fn get_recipe(id: i32, connection: &RecipeDatabase) -> Option<Recipe> {
    let recipe = repository::schema::categories::dsl::categories
        .inner_join(repository::schema::rezepte::dsl::rezepte)
        .select((
            repository::schema::rezepte::name,
            repository::schema::rezepte::ingredients,
            repository::schema::rezepte::preparation,
            repository::schema::categories::name,
            repository::schema::rezepte::number_people,
            repository::schema::rezepte::experience,
            repository::schema::rezepte::created,
            repository::schema::rezepte::rights,
            repository::schema::rezepte::owner,
            repository::schema::rezepte::time_need,
        ))
        .filter(repository::schema::rezepte::id.eq(id))
        .load::<Recipe>(&**connection)
        .unwrap()
        .first()?
        .clone();
    Some(recipe)
}

pub fn save_recipe(recipe: &NewRecipe, connection: &RecipeDatabase) -> QueryResult<usize> {
    diesel::insert_into(rezepte::table)
        .values(recipe)
        .execute(&**connection)
}
