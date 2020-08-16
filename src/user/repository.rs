use common::repository::RecipeDatabase;
use common::schema::user;
use diesel::prelude::*;
use diesel::result::Error;
use user::model::RecipeUser;

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
