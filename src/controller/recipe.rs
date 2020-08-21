use bigdecimal::BigDecimal;
use controller;
use controller::common::{CommonContext, MessageType, User};
use domain::category::Category;
use domain::recipe::RecipeName;
use domain::recipe::{NewRecipe, Recipe};
use repository;
use repository::common::RecipeDatabase;
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use std::str::FromStr;

#[derive(FromForm)]
pub struct CreateRecipe {
    name: String,
    ingredients: String,
    preparation: String,
    category: String,
    number_people: String,
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
    let new_recipe = NewRecipe {
        name: new_recipe.0.name,
        ingredients: new_recipe.0.ingredients,
        preparation: new_recipe.0.preparation,
        category: Some(new_recipe.0.category.parse::<i32>().unwrap()),
        number_people: Some(BigDecimal::from_str(&new_recipe.0.number_people).unwrap()),
        experience: None,
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
pub fn user_new_recipe(current_user: User, connection: RecipeDatabase) -> Template {
    let categories = repository::category::get_categories(&connection);
    let context = NewRecipeModel {
        categories,
        common: CommonContext {
            current_user: Some(current_user),
            message: None,
            message_type: MessageType::None,
        },
    };
    Template::render("new_recipe", &context)
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

#[derive(Serialize)]
struct NewRecipeModel {
    categories: Vec<Category>,
    common: CommonContext,
}

fn convert_newline(mut recipe: Recipe) -> Recipe {
    recipe.ingredients = recipe.ingredients.replace("\n", "<br />");
    recipe.preparation = recipe.preparation.replace("\n", "<br />");
    recipe
}
