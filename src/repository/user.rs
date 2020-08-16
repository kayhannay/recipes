use diesel::prelude::*;
use diesel::result::Error;
use domain::user::RecipeUser;
use repository::common::RecipeDatabase;
use repository::schema::user;

pub fn get_user(username: &str, connection: &RecipeDatabase) -> Option<RecipeUser> {
    let user = user::table
        .select((user::username, user::password, user::name))
        .filter(&user::username.eq(username))
        .load::<RecipeUser>(&**connection)
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
