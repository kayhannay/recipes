use bigdecimal::BigDecimal;
use controller;
use controller::common::{CommonContext, MessageType, User, create_common_context};
use domain::category::Category;
use domain::recipe::{RecipeName, UpdateRecipe};
use domain::recipe::{NewRecipe, Recipe};
use repository;
use repository::common::RecipeDatabase;
use rocket::http::Cookies;
use rocket::request::{Form, FlashMessage};
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

#[derive(Debug, FromForm)]
pub struct UpdateRecipeForm {
    id: i32,
    name: String,
    ingredients: String,
    preparation: String,
    category: Option<i32>,
    number_people: String,
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!(recipe_list))
}

#[get("/recipelist")]
pub fn recipe_list(connection: RecipeDatabase, cookies: Cookies) -> Template {
    let recipe_list = repository::recipe::get_recipes(&connection);
    let categories = repository::category::get_categories(&connection);
    let current_user = controller::common::get_current_user(cookies);
    let recipe_list_model = RecipeOverviewModel {
        recipe_names: recipe_list,
        categories,
        current_category: 0,
        common: CommonContext {
            current_user,
            message: None,
            message_type: MessageType::None,
        },
    };
    Template::render("recipe_list", &recipe_list_model)
}

#[get("/recipelist/<category>")]
pub fn recipe_list_by_category(
    category: i32,
    connection: RecipeDatabase,
    cookies: Cookies,
) -> Template {
    let recipe_list = repository::recipe::get_recipes_by_category(category, &connection);
    let categories = repository::category::get_categories(&connection);
    let current_user = controller::common::get_current_user(cookies);
    let recipe_list_model = RecipeOverviewModel {
        recipe_names: recipe_list,
        categories,
        current_category: category,
        common: CommonContext {
            current_user,
            message: None,
            message_type: MessageType::None,
        },
    };
    Template::render("recipe_list", &recipe_list_model)
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

#[get("/recipe/update/<id>")]
pub fn update_recipe_form(id: i32, current_user: User, flash: Option<FlashMessage>, connection: RecipeDatabase) -> Template {
    let categories = repository::category::get_categories(&connection);
    let recipe = repository::recipe::get_recipe(id, &connection);
    let context = UpdateRecipeModel {
        recipe: recipe.unwrap(),
        categories,
        common: create_common_context(flash, Some(current_user)),
    };
    Template::render("update_recipe", &context)
}

#[post("/recipe/update", data = "<update_recipe>")]
pub fn update_recipe(_user: User, update_recipe: Form<UpdateRecipeForm>, connection: RecipeDatabase) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let update = update_recipe.0;
    log::info!("Recipe form: {:?}", update);
    let recipe = UpdateRecipe {
        id: update.id,
        name: Some(update.name),
        ingredients: Some(update.ingredients),
        preparation: Some(update.preparation),
        category: update.category,
        number_people: Some(BigDecimal::from_str(&update.number_people).unwrap()),
        experience: None,
        rights: None,
        owner: None,
        time_need: None,
    };
    let result = repository::recipe::update_recipe(&recipe, &connection);
    match result {
        Ok(_) => {
            log::info!("Updated recipe {}", &recipe.name.unwrap());
            Ok(Flash::success(
                Redirect::to(uri!(controller::recipe::recipe_list)),
                "Recipe updated",
            ))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(controller::recipe::update_recipe_form: recipe.id)),
            "Could not update recipe!",
        )),
    }
}

#[get("/recipe/delete/<id>")]
pub fn delete_recipe(
    id: i32,
    connection: RecipeDatabase,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let result = repository::recipe::delete_recipe(id, &connection);
    match result {
        Ok(1) => {
            log::info!("Deleted recipe {}", id);
            Ok(Flash::success(
                Redirect::to(uri!(recipe_list)),
                "Recipe deleted",
            ))
        }
        _ => Err(Flash::error(
            Redirect::to(uri!(recipe: id)),
            "Could not delete recipe!",
        )),
    }
}

#[derive(Serialize)]
struct RecipeOverviewModel {
    recipe_names: Vec<RecipeName>,
    categories: Vec<Category>,
    current_category: i32,
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

#[derive(Serialize)]
struct UpdateRecipeModel {
    recipe: Recipe,
    categories: Vec<Category>,
    common: CommonContext,
}

fn convert_newline(mut recipe: Recipe) -> Recipe {
    recipe.ingredients = recipe.ingredients.replace("\n", "<br />");
    recipe.preparation = recipe.preparation.replace("\n", "<br />");
    recipe
}
