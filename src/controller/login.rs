use controller;
use controller::common::User;
use repository;
use repository::common::RecipeDatabase;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{FlashMessage, Form, FromRequest};
use rocket::response::{Flash, Redirect};
use rocket::{request, Request};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        controller::common::get_current_user(request.cookies()).or_forward(())
    }
}

#[get("/login")]
pub fn login_user(_user: User) -> Redirect {
    Redirect::to(uri!(controller::recipe::recipe_list))
}

#[get("/login", rank = 2)]
pub fn login_page(flash: Option<FlashMessage>) -> Template {
    Template::render(
        "login",
        &controller::common::create_common_context(flash, None),
    )
}

#[post("/login", data = "<login>")]
pub fn login(
    connection: RecipeDatabase,
    mut cookies: Cookies,
    login: Form<Login>,
) -> Result<Redirect, Flash<Redirect>> {
    let username = &login.username.clone();
    let recipe_user = repository::user::get_user(username, &connection);
    let error = Err(Flash::error(
        Redirect::to(uri!(login_page)),
        "Login failed!",
    ));
    if recipe_user.is_none() {
        log::info!("Failed login of user {}", username);
        return error;
    }
    let user = recipe_user.unwrap();
    if controller::common::create_hash(&login.password) == user.password {
        cookies.add_private(Cookie::new(
            controller::common::COOKIE_NAME,
            user.name.unwrap(),
        ));
        log::info!("Successful login of user {}", user.username);
        Ok(Redirect::to(uri!(controller::recipe::recipe_list)))
    } else {
        log::info!("Failed login of user {}", user.username);
        error
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named(controller::common::COOKIE_NAME));
    log::debug!("Successful logout of some user");
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}