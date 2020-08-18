extern crate chrono;
extern crate rocket;
extern crate testcontainers;

mod common;

use chrono::Utc;
use rocket::http::Status;

fn create_test_recipe() -> recipes::domain::recipe::Recipe {
    recipes::domain::recipe::Recipe {
        id: 123,
        name: "Test Recipe".to_string(),
        ingredients: "Some sugar".to_string(),
        preparation: "Boil it.".to_string(),
        experience: None,
        time_need: None,
        number_people: None,
        created: Utc::now().naive_utc(),
        owner: None,
        rights: None,
        category: None,
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
    let recipe = create_test_recipe();
    recipes::repository::recipe::save_recipe(&recipe, database_connection);

    // When
    let mut response = client.get("/").dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body_string().unwrap().contains(&format!(
        "<li class=\"recipe\"><a href=\"/recipe/{}\">{}</a></li>",
        recipe.id, recipe.name
    )));
}

#[test]
fn should_render_recipe() {
    // Given
    let (client, database_connection) = common::setup();
    let recipe = create_test_recipe();
    recipes::repository::recipe::save_recipe(&recipe, database_connection);

    // When
    let mut response = client.get(format!("/recipe/{}", recipe.id)).dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .body_string()
        .unwrap()
        .contains(&format!("<h3>{}</h3>", recipe.name)));
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
    let user = recipes::domain::user::RecipeUser {
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
