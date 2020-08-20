use diesel::prelude::*;
use diesel::result::Error;
use domain::category::{Category, NewCategory};
use repository::common::RecipeDatabase;
use repository::schema::categories;
use repository::schema::categories::all_columns;

pub fn get_categories(connection: &RecipeDatabase) -> Vec<Category> {
    categories::table
        .select(all_columns)
        .load::<Category>(&**connection)
        .unwrap()
}

pub fn save_category(category: &NewCategory, connection: &RecipeDatabase) -> Result<usize, Error> {
    diesel::insert_into(categories::table)
        .values(category)
        .execute(&**connection)
}
