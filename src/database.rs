use model::{RecipeName, Recipe, RecipeUser};
use schema::{rezepte, user};
use diesel::prelude::*;
use schema::rezepte::all_columns;
use diesel::result::Error;

embed_migrations!();

#[database("recipe_db")]
pub struct RecipeDatabase(diesel::MysqlConnection);

pub fn run_migrations(connection: &diesel::MysqlConnection) {
    embedded_migrations::run_with_output(connection, &mut std::io::stdout()).expect("Could not run database migrations!");
}

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

pub fn get_user(username: String, connection: RecipeDatabase) -> Option<RecipeUser> {
    let user = user::table
        .select((user::username, user::password, user::name))
        .filter(&user::username.eq(username))
        .load::<RecipeUser>(&*connection)
        .unwrap()
        .first()?
        .clone();
    Some(user)
}

pub fn save_user(user: &RecipeUser, connection: RecipeDatabase) -> Result<usize, Error> {
    diesel::insert_into(user::table)
        .values(user)
        .execute(&*connection)
}
