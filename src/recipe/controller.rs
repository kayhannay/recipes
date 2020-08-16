use common;
use common::controller::{CommonContext, MessageType};
use common::repository::RecipeDatabase;
use recipe::model::Recipe;
use recipe::model::RecipeName;
use recipe::repository;
use rocket::http::Cookies;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn recipe_list(connection: RecipeDatabase, cookies: Cookies) -> Template {
    let recipe_list = repository::get_recipes(connection);
    let current_user = common::controller::get_current_user(cookies);
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
    let recipe = repository::get_recipe(id, connection);
    let converted_recipe = convert_newline(recipe?);
    let current_user = common::controller::get_current_user(cookies);
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
