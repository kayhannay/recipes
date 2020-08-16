use diesel::prelude::*;
use user::model::RecipeUser;
use common::schema::user;
use common::repository::RecipeDatabase;
use diesel::result::Error;

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
