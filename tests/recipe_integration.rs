extern crate bigdecimal;
extern crate chrono;
extern crate recipes;
extern crate rocket;
extern crate testcontainers;

mod common;

use bigdecimal::BigDecimal;
use recipes::domain::category::NewCategory;
use recipes::repository::common::RecipeDatabase;
use rocket::http::{ContentType, Status};

fn create_test_recipe(database_connection: &RecipeDatabase) -> recipes::domain::recipe::NewRecipe {
    recipes::repository::category::save_category(
        &NewCategory {
            name: "Testcategory".to_string(),
        },
        &database_connection,
    )
    .ok();
    let categories = recipes::repository::category::get_categories(&database_connection);
    println!("Categories: {:?}", categories);
    let category = categories.get(0).unwrap();
    println!("Category: {:?}", Option::from(category.id));
    recipes::domain::recipe::NewRecipe {
        name: "Test Recipe".to_string(),
        ingredients: "Some sugar".to_string(),
        preparation: "Boil it.".to_string(),
        experience: None,
        time_need: None,
        number_people: None,
        owner: None,
        rights: None,
        category: Some(category.id),
    }
}

#[test]
fn should_render_empty_recipe_list() {
    // Given
    let (client, _database_connection) = common::setup();

    // When
    let mut response = client.get("/").dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(!response
        .body_string()
        .unwrap()
        .contains("<li class=\"recipe\">"));
}

#[test]
fn should_render_recipe_list() {
    // Given
    let (client, database_connection) = common::setup();
    let recipe = create_test_recipe(&database_connection);
    recipes::repository::recipe::save_recipe(&recipe, &database_connection).ok();
    let recipes = recipes::repository::recipe::get_recipes(&database_connection);
    let recipe_id = recipes.get(0).unwrap().id;

    // When
    let mut response = client.get("/").dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body_string().unwrap().contains(&format!(
        "<li class=\"recipe\"><a href=\"/recipe/{}\">{}</a></li>",
        recipe_id, recipe.name
    )));
}

#[test]
fn should_render_recipe() {
    // Given
    let (client, database_connection) = common::setup();
    let recipe = create_test_recipe(&database_connection);
    recipes::repository::recipe::save_recipe(&recipe, &database_connection).ok();
    let recipes = recipes::repository::recipe::get_recipes(&database_connection);
    let recipe_id = recipes.get(0).unwrap().id;

    // When
    let mut response = client.get(format!("/recipe/{}", recipe_id)).dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .body_string()
        .unwrap()
        .contains(&format!("<h2 id=\"recipe-name\">{}</h2>", recipe.name)));
}

#[test]
fn should_return_404_for_missing_recipe() {
    // Given
    let (client, _database_connection) = common::setup();

    // When
    let response = client.get("/recipe/22").dispatch();

    // Then
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn should_redirect_anonymous() {
    // Given
    let (client, _) = common::setup();

    // When
    let response = client.get("/newrecipe").dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn should_render_new_recipe_form() {
    // Given
    let (client, database_connection) = common::setup();
    let password = "geheim";
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: recipes::controller::common::create_hash(password),
        name: None,
    };
    recipes::repository::user::save_user(&user, &database_connection).ok();
    let login_cookie = common::login(&client, &user.username, password).expect("logged in");

    // When
    let mut response = client
        .get("/newrecipe")
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .body_string()
        .unwrap()
        .contains("<title>Rezept - Neu</title>"));
}

#[test]
fn should_create_recipe() {
    // Given
    let (client, database_connection) = common::setup();
    let password = "geheim";
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: recipes::controller::common::create_hash(password),
        name: None,
    };
    recipes::repository::user::save_user(&user, &database_connection).ok();
    let login_cookie = common::login(&client, &user.username, password).expect("logged in");
    let db_user =
        recipes::repository::user::get_user(&user.username, &database_connection).unwrap();
    let test_recipe = create_test_recipe(&database_connection);

    // When
    let response = client
        .post("/recipe")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!(
            "name={}&ingredients={}&preparation={}&category={}",
            test_recipe.name,
            test_recipe.ingredients,
            test_recipe.preparation,
            test_recipe.category.unwrap()
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));
    let recipes = recipes::repository::recipe::get_recipes(&database_connection);
    assert_eq!(recipes.len(), 1);
    println!("Recipes: {:?}", recipes);
    let result_recipe =
        recipes::repository::recipe::get_recipe(recipes.get(0).unwrap().id, &database_connection)
            .unwrap();
    assert_eq!(result_recipe.name, test_recipe.name);
    assert_eq!(result_recipe.ingredients, test_recipe.ingredients);
    assert_eq!(result_recipe.preparation, test_recipe.preparation);
    assert_eq!(result_recipe.owner, Some(BigDecimal::from(db_user.uid)));
}
