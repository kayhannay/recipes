use bigdecimal::BigDecimal;
use chrono::Utc;
use controller;
use controller::common::{CommonContext, MessageType, User};
use domain::recipe::Recipe;
use domain::recipe::RecipeName;
use repository;
use repository::common::RecipeDatabase;
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct CreateRecipe {
    name: String,
    ingredients: String,
    preparation: String,
}

#[get("/")]
pub fn recipe_list(connection: RecipeDatabase, cookies: Cookies) -> Template {
    let recipe_list = repository::recipe::get_recipes(&connection);
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
    let recipe = repository::recipe::get_recipe(id, &connection);
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

#[post("/recipe", data = "<new_recipe>")]
pub fn create_recipe(
    user: User,
    new_recipe: Form<CreateRecipe>,
    connection: RecipeDatabase,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let new_recipe = Recipe {
        name: new_recipe.0.name,
        ingredients: new_recipe.0.ingredients,
        preparation: new_recipe.0.preparation,
        category: None,
        number_people: None,
        experience: None,
        created: Utc::now().naive_utc(),
        rights: None,
        owner: Some(BigDecimal::from(user.uid)),
        time_need: None,
    };
    let _ = repository::recipe::save_recipe(&new_recipe, &connection);
    Ok(Flash::success(
        Redirect::to(uri!(recipe_list)),
        "Recipe created",
    ))
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
