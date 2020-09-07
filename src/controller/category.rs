use controller;
use controller::common::{create_common_context, CommonContext, User};
use domain::category::{Category, NewCategory};
use repository;
use repository::common::RecipeDatabase;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct CreateCategory {
    name: String,
}

#[derive(FromForm)]
pub struct UpdateCategory {
    id: i32,
    name: String,
}

#[post("/category", data = "<new_category>")]
pub fn category_create_user(
    _user: User,
    connection: RecipeDatabase,
    new_category: Form<CreateCategory>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let category = new_category.0;
    let result = repository::category::save_category(
        &NewCategory {
            name: category.name.clone(),
        },
        &connection,
    );
    match result {
        Ok(_) => {
            log::info!("Created category {}", &category.name);
            Ok(Flash::success(
                Redirect::to(uri!(controller::config::user_config)),
                "Category created",
            ))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(controller::config::user_config)),
            "Could not create category!",
        )),
    }
}

#[derive(Serialize)]
struct CategoryModel {
    category: Category,
    common: CommonContext,
}

#[get("/category/update/<id>")]
pub fn category_update_form_user(
    id: i32,
    user: User,
    flash: Option<FlashMessage>,
    connection: RecipeDatabase,
) -> Option<Template> {
    let category_result = repository::category::get_category(id, &connection)?;
    let context = CategoryModel {
        category: category_result,
        common: create_common_context(flash, Some(user)),
    };
    Some(Template::render("update_category", &context))
}

#[get("/category/update/<_id>", rank = 2)]
pub fn category_update_form(_id: i32) -> Redirect {
    Redirect::to(uri!(controller::login::login_page))
}

#[post("/category/update", data = "<update_category>")]
pub fn category_update_user(
    _user: User,
    connection: RecipeDatabase,
    update_category: Form<UpdateCategory>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let category = update_category.0;
    let result = repository::category::update_category(
        &Category {
            id: category.id,
            name: category.name.clone(),
        },
        &connection,
    );
    match result {
        Ok(_) => {
            log::info!("Updated category {}", &category.name);
            Ok(Flash::success(
                Redirect::to(uri!(controller::config::user_config)),
                "Category updated",
            ))
        }
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(controller::config::user_config)),
            "Could not update category!",
        )),
    }
}

#[get("/category/delete/<id>")]
pub fn category_delete_user(
    id: i32,
    _user: User,
    connection: RecipeDatabase,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let result = repository::category::delete_category(id, &connection);
    match result {
        Ok(1) => {
            log::info!("Deleted category {}", id);
            Ok(Flash::success(
                Redirect::to(uri!(controller::config::user_config)),
                "Category deleted",
            ))
        }
        _ => Err(Flash::error(
            Redirect::to(uri!(controller::config::user_config)),
            "Could not delete category!",
        )),
    }
}

#[get("/category/delete/<_id>", rank = 2)]
pub fn category_delete(_id: i32) -> Redirect {
    Redirect::to(uri!(controller::login::login_page))
}
