extern crate crypto;

use rocket::request::{FromRequest, Form, FlashMessage};
use rocket::{Request, request};
use rocket::outcome::IntoOutcome;
use rocket::http::{Cookies, Cookie};
use rocket::response::{Redirect, Flash};
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use request_handler;
use database;
use database::RecipeDatabase;
use std::str;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;
use model::RecipeUser;

const COOKIE_NAME: &str = "user";

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String
}

#[derive(FromForm)]
pub struct CreateUser {
    username: String,
    password: String,
    name: Option<String>
}

#[derive(Debug, Serialize)]
pub struct User(String);

#[derive(Debug, Serialize)]
pub enum MessageType {
    ERROR,
    WARN,
    INFO,
    None
}

#[derive(Debug, Serialize)]
pub struct CommonContext {
    pub current_user: Option<User>,
    pub message: Option<String>,
    pub message_type: MessageType
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        get_current_user(request.cookies())
            .or_forward(())
    }
}

#[post("/login", data = "<login>")]
pub fn login(connection: RecipeDatabase, mut cookies: Cookies, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    let username = &login.username.clone();
    let recipe_user = database::get_user(username, &connection);
    let error = Err(Flash::error(Redirect::to(uri!(login_page)), "Login failed!"));
    if recipe_user.is_none() {
        log::info!("Failed login of user {}", username);
        return error;
    }
    let user = recipe_user.unwrap();
    if create_hash(&login.password) == user.password {
        cookies.add_private(Cookie::new(COOKIE_NAME, user.name.unwrap()));
        log::info!("Successful login of user {}", user.username);
        Ok(Redirect::to(uri!(request_handler::recipe_list)))
    } else{
        log::info!("Failed login of user {}", user.username);
        error
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named(COOKIE_NAME));
    log::debug!("Successful logout of some user");
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/login")]
pub fn login_user(_user: User) -> Redirect {
    Redirect::to(uri!(request_handler::recipe_list))
}

fn create_common_context<'a>(flash: Option<FlashMessage>, user: Option<User>) -> HashMap<&'a str, CommonContext> {
    let mut context = HashMap::new();
    let mut common = CommonContext { current_user: user, message: None , message_type: MessageType::None};
    if let Some(ref msg) = flash {
        let message_type = match msg.name() {
            "error" => MessageType::ERROR,
            "warning" => MessageType::WARN,
            _ => MessageType::INFO
        };
        common.message = Some(msg.msg().to_string());
        common.message_type = message_type;
        context.insert("common", common);
    }
    context
}

#[get("/login", rank = 2)]
pub fn login_page(flash: Option<FlashMessage>) -> Template {
    Template::render("login", &create_common_context(flash, None))
}

#[get("/config")]
pub fn user_config(user: User, flash: Option<FlashMessage>) -> Template {
    Template::render("config", &create_common_context(flash, Some(user)))
}

#[get("/config", rank = 2)]
pub fn config() -> Redirect {
    Redirect::to(uri!(login_page))
}

#[post("/user", data = "<new_user>")]
pub fn create_user(connection: RecipeDatabase, new_user: Form<CreateUser>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let login_user = new_user.0;
    let error = Err(Flash::error(Redirect::to(uri!(user_config)), "Could not create user!"));
    if login_user.username.is_empty() || login_user.password.is_empty() {
        return error;
    }
    let recipe_user = RecipeUser {
        username: login_user.username,
        password: create_hash(&login_user.password),
        name: login_user.name
    };
    let result = database::save_user(&recipe_user, connection);
    match result {
        Ok(_) => {
            log::info!("Created user {}", &recipe_user.username);
            Ok(Flash::success(Redirect::to(uri!(user_config)), "User created"))
        },
        Err(_) => error
    }
}

pub fn get_current_user(mut cookies: Cookies) -> Option<User> {
    cookies
        .get_private(COOKIE_NAME)
        .and_then(|cookie| cookie.value().parse().ok())
        .map(User)
}

fn create_hash(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input_str(input);
    hasher.result_str()
}