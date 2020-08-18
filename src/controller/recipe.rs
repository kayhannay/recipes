use controller;
use controller::common::{CommonContext, MessageType, User};
use domain::recipe::Recipe;
use domain::recipe::RecipeName;
use repository;
use repository::common::RecipeDatabase;
use rocket::http::Cookies;
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn recipe_list(connection: RecipeDatabase, cookies: Cookies) -> Template {
    let recipe_list = repository::recipe::get_recipes(connection);
    let current_user = controller::common::get_current_user(cookies);
    let recipe_list_model = RecipeOverviewModel {
        recipe_names: recipe_list,
        common: CommonContext {
            current_user,
            message: None,
            message_type: MessageType::None,
        },
    };
    Template::render("index", &recipe_list_model)
}

#[get("/recipe/<id>")]
pub fn recipe(id: i32, connection: RecipeDatabase, cookies: Cookies) -> Option<Template> {
    let recipe = repository::recipe::get_recipe(id, connection);
    let converted_recipe = convert_newline(recipe?);
    let current_user = controller::common::get_current_user(cookies);
    let model = RecipeModel {
        recipe: converted_recipe,
        common: CommonContext {
            current_user,
            message: None,
            message_type: MessageType::None,
        },
    };
    Some(Template::render("recipe", &model))
}

#[get("/newrecipe")]
pub fn user_new_recipe(user: User, flash: Option<FlashMessage>) -> Template {
    Template::render(
        "new_recipe",
        &controller::common::create_common_context(flash, Some(user)),
    )
}

#[get("/newrecipe", rank = 2)]
pub fn new_recipe() -> Redirect {
    Redirect::to(uri!(controller::login::login_page))
}

#[derive(Serialize)]
struct RecipeOverviewModel {
    recipe_names: Vec<RecipeName>,
    common: CommonContext,
}

#[derive(Serialize)]
struct RecipeModel {
    recipe: Recipe,
    common: CommonContext,
}

fn convert_newline(mut recipe: Recipe) -> Recipe {
    recipe.ingredients = recipe.ingredients.replace("\n", "<br />");
    recipe.preparation = recipe.preparation.replace("\n", "<br />");
    recipe
}
