extern crate crypto;

use controller;
use domain::user::NewRecipeUser;
use repository;
use repository::common::RecipeDatabase;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

#[derive(FromForm)]
pub struct CreateUser {
    username: String,
    password: String,
    name: String,
}

#[post("/user", data = "<new_user>")]
pub fn create_user(
    connection: RecipeDatabase,
    new_user: Form<CreateUser>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let login_user = new_user.0;
    let error = Err(Flash::error(
        Redirect::to(uri!(controller::config::user_config)),
        "Could not create user!",
    ));
    if login_user.username.is_empty() || login_user.password.is_empty() {
        return error;
    }
    let name = if login_user.name.is_empty() {
        None
    } else {
        Some(login_user.name)
    };
    let recipe_user = NewRecipeUser {
        username: login_user.username,
        password: controller::common::create_hash(&login_user.password),
        name,
    };
    let result = repository::user::save_user(&recipe_user, &connection);
    match result {
        Ok(_) => {
            log::info!("Created user {}", &recipe_user.username);
            Ok(Flash::success(
                Redirect::to(uri!(controller::config::user_config)),
                "User created",
            ))
        }
        Err(_) => error,
    }
}
