use controller;
use domain::category::NewCategory;
use repository;
use repository::common::RecipeDatabase;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

#[derive(FromForm)]
pub struct CreateCategory {
    name: String,
}

#[post("/category", data = "<new_category>")]
pub fn create_category(
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

#[get("/deletecategory/<id>")]
pub fn delete_category(
    id: i32,
    connection: RecipeDatabase,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let result = repository::category::delete_category(id, &connection);
    log::info!("Delete result: {:?}", result);
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
