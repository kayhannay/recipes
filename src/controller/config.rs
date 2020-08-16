use controller;
use controller::common::User;
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/config")]
pub fn user_config(user: User, flash: Option<FlashMessage>) -> Template {
    Template::render(
        "config",
        &controller::common::create_common_context(flash, Some(user)),
    )
}

#[get("/config", rank = 2)]
pub fn config() -> Redirect {
    Redirect::to(uri!(controller::login::login_page))
}
