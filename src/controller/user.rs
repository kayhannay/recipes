extern crate crypto;

use controller;
use controller::common::{create_common_context, CommonContext, User};
use domain::user::{NewRecipeUser, RecipeUser, UpdateRecipeUser};
use repository;
use repository::common::RecipeDatabase;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct CreateUser {
    username: String,
    password: String,
    name: String,
}

#[derive(FromForm)]
pub struct UpdateUser {
    id: i32,
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

#[derive(Serialize)]
struct UserModel {
    user: RecipeUser,
    common: CommonContext,
}

#[get("/update-user/<id>")]
pub fn update_user_form(
    id: i32,
    user: User,
    flash: Option<FlashMessage>,
    connection: RecipeDatabase,
) -> Option<Template> {
    let mut user_result = repository::user::get_user(id, &connection)?;
    user_result.password = "".to_string();
    let context = UserModel {
        user: user_result,
        common: create_common_context(flash, Some(user)),
    };
    Some(Template::render("update_user", &context))
}

#[post("/update-user", data = "<update_user>")]
pub fn update_user(
    _user: User,
    connection: RecipeDatabase,
    update_user: Form<UpdateUser>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let user = update_user.0;
    let result = repository::user::update_user(
        &UpdateRecipeUser {
            id: user.id,
            username: Some(user.username).filter(|x| !x.is_empty()),
            password: Some(user.password).filter(|x| !x.is_empty()),
            name: Some(user.name.clone()).filter(|x| !x.is_empty()),
        },
        &connection,
    );
    match result {
        Ok(_) => {
            log::info!("Updated user {}", &user.name);
            Ok(Flash::success(
                Redirect::to(uri!(controller::config::user_config)),
                "User updated",
            ))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(controller::config::user_config)),
            "Could not update user!",
        )),
    }
}

#[get("/deleteuser/<id>")]
pub fn delete_user(
    id: i32,
    connection: RecipeDatabase,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let result = repository::user::delete_user(id, &connection);
    match result {
        Ok(1) => {
            log::info!("Deleted user {}", id);
            Ok(Flash::success(
                Redirect::to(uri!(controller::config::user_config)),
                "User deleted",
            ))
        }
        _ => Err(Flash::error(
            Redirect::to(uri!(controller::config::user_config)),
            "Could not delete user!",
        )),
    }
}
