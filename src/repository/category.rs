use diesel::prelude::*;
use diesel::result::Error;
use domain::category::{Category, NewCategory};
use repository;
use repository::common::RecipeDatabase;
use repository::schema::categories;
use repository::schema::categories::all_columns;

pub fn get_categories(connection: &RecipeDatabase) -> Vec<Category> {
    let mut categories: Vec<Category> = categories::table
        .select(all_columns)
        .load::<Category>(&**connection)
        .unwrap();
    categories.sort_by(|a, b| a.name.cmp(&b.name));
    categories
}

fn get_category(category_name: &str, connection: &RecipeDatabase) -> Option<Category> {
    let category = categories::table
        .select(all_columns)
        .filter(categories::name.eq(category_name))
        .load::<Category>(&**connection)
        .unwrap()
        .first()?
        .clone();
    Some(category)
}

pub fn save_category(category: &NewCategory, connection: &RecipeDatabase) -> Result<usize, Error> {
    if get_category(&category.name, connection).is_some() {
        Err(Error::NotFound)
    } else {
        diesel::insert_into(categories::table)
            .values(category)
            .execute(&**connection)
    }
}

pub fn delete_category(category_id: i32, connection: &RecipeDatabase) -> Result<usize, Error> {
    diesel::delete(
        repository::schema::categories::dsl::categories.filter(categories::id.eq(category_id)),
    )
    .execute(&**connection)
}
