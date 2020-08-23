use diesel::prelude::*;
use diesel::result::Error;
use domain::user::{NewRecipeUser, RecipeUser};
use repository;
use repository::common::RecipeDatabase;
use repository::schema::user;

pub fn get_user(username: &str, connection: &RecipeDatabase) -> Option<RecipeUser> {
    let user = user::table
        .select((user::uid, user::username, user::password, user::name))
        .filter(&user::username.eq(username))
        .load::<RecipeUser>(&**connection)
        .unwrap()
        .first()?
        .clone();
    Some(user)
}

pub fn get_all_user(connection: &RecipeDatabase) -> Vec<RecipeUser> {
    let mut user: Vec<RecipeUser> = user::table
        .select((user::uid, user::username, user::password, user::name))
        .load::<RecipeUser>(&**connection)
        .unwrap();
    user.sort();
    user
}

pub fn save_user(user: &NewRecipeUser, connection: &RecipeDatabase) -> Result<usize, Error> {
    if get_user(&user.username, connection).is_some() {
        Err(Error::NotFound)
    } else {
        diesel::insert_into(user::table)
            .values(user)
            .execute(&**connection)
    }
}

pub fn delete_user(user_id: i32, connection: &RecipeDatabase) -> Result<usize, Error> {
    diesel::delete(repository::schema::user::dsl::user.filter(user::uid.eq(user_id)))
        .execute(&**connection)
}
