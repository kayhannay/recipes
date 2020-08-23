use controller::common::{CommonContext, User};
use domain::category::Category;
use domain::user::RecipeUser;
use repository::common::RecipeDatabase;
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use {controller, repository};

#[derive(Serialize)]
struct ConfigModel {
    user: Vec<RecipeUser>,
    categories: Vec<Category>,
    common: CommonContext,
}

#[get("/config")]
pub fn user_config(
    user: User,
    flash: Option<FlashMessage>,
    connection: RecipeDatabase,
) -> Template {
    let context = ConfigModel {
        user: repository::user::get_all_user(&connection),
        categories: repository::category::get_categories(&connection),
        common: controller::common::create_common_context(flash, Some(user)),
    };

    Template::render("config", &context)
}

#[get("/config", rank = 2)]
pub fn config() -> Redirect {
    Redirect::to(uri!(controller::login::login_page))
}
