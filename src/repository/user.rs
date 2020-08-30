use diesel::prelude::*;
use diesel::result::Error;
use domain::user::{NewRecipeUser, RecipeUser, UpdateRecipeUser};
use repository;
use repository::common::RecipeDatabase;
use repository::schema::user;

pub fn get_user_by_name(username: &str, connection: &RecipeDatabase) -> Option<RecipeUser> {
    let user = user::table
        .select((user::id, user::username, user::password, user::name))
        .filter(&user::username.eq(username))
        .load::<RecipeUser>(&**connection)
        .unwrap()
        .first()?
        .clone();
    Some(user)
}

pub fn get_user(user_id: i32, connection: &RecipeDatabase) -> Option<RecipeUser> {
    let user = user::table
        .select((user::id, user::username, user::password, user::name))
        .filter(&user::id.eq(user_id))
        .load::<RecipeUser>(&**connection)
        .unwrap()
        .first()?
        .clone();
    Some(user)
}

pub fn get_all_user(connection: &RecipeDatabase) -> Vec<RecipeUser> {
    let mut user: Vec<RecipeUser> = user::table
        .select((user::id, user::username, user::password, user::name))
        .load::<RecipeUser>(&**connection)
        .unwrap();
    user.sort();
    user
}

pub fn save_user(user: &NewRecipeUser, connection: &RecipeDatabase) -> Result<usize, Error> {
    if get_user_by_name(&user.username, connection).is_some() {
        Err(Error::NotFound)
    } else {
        diesel::insert_into(user::table)
            .values(user)
            .execute(&**connection)
    }
}

pub fn update_user(user: &UpdateRecipeUser, connection: &RecipeDatabase) -> Result<usize, Error> {
    let existing_user = get_user_by_name(&user.username.clone().unwrap(), connection);
    log::info!("Existing user: {:?}, update: {:?}", existing_user, user);
    if existing_user.is_some() && existing_user.unwrap().id != user.id {
        log::info!("Error");
        Err(Error::NotFound)
    } else {
        log::info!("Update ...");
        let res = diesel::update(user).set(user).execute(&**connection);
        log::info!("Result: {:?}", res);
        res
    }
}

pub fn delete_user(user_id: i32, connection: &RecipeDatabase) -> Result<usize, Error> {
    diesel::delete(repository::schema::user::dsl::user.filter(user::id.eq(user_id)))
        .execute(&**connection)
}
