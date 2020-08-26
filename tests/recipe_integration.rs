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
use std::collections::HashMap;

const TEST_CATEGORY: &str = "Testcategory";

fn create_test_recipe(database_connection: &RecipeDatabase) -> recipes::domain::recipe::NewRecipe {
    create_test_recipe_by_category(TEST_CATEGORY, "Test Recipe", database_connection)
}

fn create_test_recipe_by_category(
    category: &str,
    recipe_name: &str,
    database_connection: &RecipeDatabase,
) -> recipes::domain::recipe::NewRecipe {
    recipes::repository::category::save_category(
        &NewCategory {
            name: String::from(category),
        },
        &database_connection,
    )
    .ok();
    let categories = recipes::repository::category::get_categories(&database_connection);
    let mut category_map = HashMap::new();
    for c in categories {
        category_map.insert(c.name.clone(), c.id);
    }
    let category_id = category_map.get(category).unwrap();
    recipes::domain::recipe::NewRecipe {
        name: recipe_name.to_string(),
        ingredients: "Some sugar".to_string(),
        preparation: "Boil it.".to_string(),
        experience: None,
        time_need: None,
        number_people: Some(BigDecimal::from(4)),
        owner: None,
        rights: None,
        category: Some(*category_id),
    }
}

#[test]
fn should_forward_to_recipelist_from_index() {
    // Given
    let (client, _) = common::setup();

    // When
    let response = client.get("/").dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/recipelist"));
}

#[test]
fn should_render_empty_recipe_list() {
    // Given
    let (client, _database_connection) = common::setup();

    // When
    let mut response = client.get("/recipelist").dispatch();

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
    let mut response = client.get("/recipelist").dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body_string().unwrap().contains(&format!(
        "<li class=\"recipe\"><a href=\"/recipe/{}\">{}</a></li>",
        recipe_id, recipe.name
    )));
}

#[test]
fn should_render_recipe_list_by_category() {
    // Given
    let (client, database_connection) = common::setup();
    let recipe_cat1 =
        create_test_recipe_by_category("Category1", "Recipe Category 1", &database_connection);
    let recipe_cat2 =
        create_test_recipe_by_category("Category2", "Recipe Category 2", &database_connection);
    recipes::repository::recipe::save_recipe(&recipe_cat1, &database_connection).ok();
    recipes::repository::recipe::save_recipe(&recipe_cat2, &database_connection).ok();

    // When
    let mut response = client
        .get(format!("/recipelist/{}", recipe_cat2.category.unwrap()))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    let body = response.body_string().unwrap();
    println!("Body is: {}", &body);
    assert!(body.contains(&format!("\">{}</a></li>", recipe_cat2.name)));
    assert!(!body.contains(&format!("\">{}</a></li>", recipe_cat1.name)));
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
        .contains("<title>Rezepte - Neu</title>"));
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
    let number_people = test_recipe.number_people.unwrap();

    // When
    let response = client
        .post("/recipe")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!(
            "name={}&ingredients={}&preparation={}&category={}&number_people={}",
            test_recipe.name,
            test_recipe.ingredients,
            test_recipe.preparation,
            test_recipe.category.unwrap(),
            number_people,
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/recipelist"));
    let recipes = recipes::repository::recipe::get_recipes(&database_connection);
    assert_eq!(recipes.len(), 1);
    println!("Recipes: {:?}", recipes);
    let result_recipe =
        recipes::repository::recipe::get_recipe(recipes.get(0).unwrap().id, &database_connection)
            .unwrap();
    assert_eq!(result_recipe.name, test_recipe.name);
    assert_eq!(result_recipe.ingredients, test_recipe.ingredients);
    assert_eq!(result_recipe.preparation, test_recipe.preparation);
    assert_eq!(result_recipe.category, TEST_CATEGORY);
    assert_eq!(result_recipe.number_people, Some(number_people));
    assert_eq!(result_recipe.owner, Some(BigDecimal::from(db_user.uid)));
}
