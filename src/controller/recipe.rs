use bigdecimal::BigDecimal;
use controller;
use controller::common::{create_common_context, CommonContext, MessageType, User};
use domain::category::Category;
use domain::recipe::{NewRecipe, Recipe};
use domain::recipe::{RecipeName, UpdateRecipe};
use repository;
use repository::common::RecipeDatabase;
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use std::str::FromStr;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!(recipe_list))
}

#[get("/recipe/list")]
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

#[get("/recipe/list/<category>")]
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
pub fn recipe_by_id(id: i32, connection: RecipeDatabase, cookies: Cookies) -> Option<Template> {
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

#[derive(FromForm)]
pub struct NewRecipeForm {
    name: String,
    ingredients: String,
    preparation: String,
    category: String,
    number_people: String,
}

#[post("/recipe", data = "<new_recipe>")]
pub fn recipe_new_user(
    user: User,
    new_recipe: Form<NewRecipeForm>,
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
    let _ = repository::recipe::create_recipe(&new_recipe, &connection);
    Ok(Flash::success(
        Redirect::to(uri!(recipe_list)),
        "Recipe created",
    ))
}

#[get("/recipe/new")]
pub fn recipe_new_form_user(current_user: User, connection: RecipeDatabase) -> Template {
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

#[get("/recipe/new", rank = 2)]
pub fn recipe_new_form() -> Redirect {
    Redirect::to(uri!(controller::login::login_page))
}

#[get("/recipe/update/<id>")]
pub fn recipe_update_form_user(
    id: i32,
    current_user: User,
    flash: Option<FlashMessage>,
    connection: RecipeDatabase,
) -> Template {
    let categories = repository::category::get_categories(&connection);
    let recipe = repository::recipe::get_recipe(id, &connection);
    let context = UpdateRecipeModel {
        recipe: recipe.unwrap(),
        categories,
        common: create_common_context(flash, Some(current_user)),
    };
    Template::render("update_recipe", &context)
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

#[post("/recipe/update", data = "<update_recipe>")]
pub fn recipe_update_user(
    _user: User,
    update_recipe: Form<UpdateRecipeForm>,
    connection: RecipeDatabase,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
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
            Redirect::to(uri!(controller::recipe::recipe_update_form_user: recipe.id)),
            "Could not update recipe!",
        )),
    }
}

#[get("/recipe/delete/<id>")]
pub fn recipe_delete_user(
    id: i32,
    _user: User,
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
            Redirect::to(uri!(recipe_by_id: id)),
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
