use diesel::prelude::*;
use diesel::result::Error;
use domain::user::{NewRecipeUser, RecipeUser};
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

pub fn save_user(user: &NewRecipeUser, connection: &RecipeDatabase) -> Result<usize, Error> {
    if get_user(&user.username, connection).is_some() {
        Err(Error::NotFound)
    } else {
        diesel::insert_into(user::table)
            .values(user)
            .execute(&**connection)
    }
}
