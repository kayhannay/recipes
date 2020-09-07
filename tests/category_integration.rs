extern crate chrono;
extern crate recipes;
extern crate rocket;
extern crate testcontainers;

mod common;

use recipes::domain::category::Category;
use rocket::http::{ContentType, Status};

#[test]
fn should_create_category() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let category = recipes::domain::category::NewCategory {
        name: "Category1".to_string(),
    };

    // When
    let response = client
        .post("/category")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!("name={}", category.name))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let result = recipes::repository::category::get_categories(&database_connection);
    assert_eq!(result.len(), 1);
    let result_category = result.get(0).unwrap();
    assert_eq!(result_category.name, category.name);
}

#[test]
fn should_require_authenticated_user_create_user() {
    // Given
    let (client, _database_connection) = common::setup();

    // When
    let response = client
        .post("/category")
        .header(ContentType::Form)
        .body("name=foo")
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn should_not_create_category() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let category = recipes::domain::category::NewCategory {
        name: "Category1".to_string(),
    };
    recipes::repository::category::save_category(&category, &database_connection).ok();

    // When
    let response = client
        .post("/category")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!("name={}", category.name))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not create category!"
    );
    let result = recipes::repository::category::get_categories(&database_connection);
    assert_eq!(result.len(), 1);
}

#[test]
fn should_redirect_anonymous_delete() {
    // Given
    let (client, _) = common::setup();

    // When
    let response = client.get("/category/delete/123").dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn should_delete_category() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let category = recipes::domain::category::NewCategory {
        name: "Category1".to_string(),
    };
    recipes::repository::category::save_category(&category, &database_connection).ok();
    let categories: Vec<Category> =
        recipes::repository::category::get_categories(&database_connection);
    let category_id = categories.get(0).unwrap().id;

    // When
    let response = client
        .get(format!("/category/delete/{}", category_id))
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(flash_cookie.unwrap().value(), "7:successCategory deleted");
    let result = recipes::repository::category::get_categories(&database_connection);
    assert_eq!(result.len(), 0);
}

#[test]
fn should_not_delete_category() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);

    // When
    let response = client
        .get(format!("/category/delete/{}", 123))
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not delete category!"
    );
    let result = recipes::repository::category::get_categories(&database_connection);
    assert_eq!(result.len(), 0);
}

#[test]
fn should_redirect_anonymous_update() {
    // Given
    let (client, _) = common::setup();

    // When
    let response = client.get("/category/update/123").dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn should_render_update_category_form() {
    // Given
    let (client, database_connection) = common::setup();
    let (_user, login_cookie) = common::create_login_user(&client, &database_connection);
    let category = recipes::domain::category::NewCategory {
        name: "Category1".to_string(),
    };
    recipes::repository::category::save_category(&category, &database_connection).ok();
    let category_id = recipes::repository::category::get_categories(&database_connection)
        .get(0)
        .unwrap()
        .id;

    // When
    let mut response = client
        .get(format!("/category/update/{}", category_id))
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    let body = response.body_string().unwrap();
    assert!(body.contains("<title>Rezepte - Kategorie</title>"));
    assert!(body.contains(&format!("value=\"{}\"", category.name)));
    assert!(body.contains(&format!("value=\"{}\"", category_id)));
}

#[test]
fn should_update_category() {
    // Given
    let (client, database_connection) = common::setup();
    let (_user, login_cookie) = common::create_login_user(&client, &database_connection);
    let category = recipes::domain::category::NewCategory {
        name: "Category1".to_string(),
    };
    recipes::repository::category::save_category(&category, &database_connection).ok();
    let category_id = recipes::repository::category::get_categories(&database_connection)
        .get(0)
        .unwrap()
        .id;
    let updated_category_name = "UpdatedCategory1";

    // When
    let response = client
        .post("/category/update")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!("id={}&name={}", category_id, updated_category_name))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let result_category =
        recipes::repository::category::get_category(category_id, &database_connection).unwrap();
    assert_eq!(result_category.name, updated_category_name);
}

#[test]
fn should_require_authenticated_user_update_user() {
    // Given
    let (client, _database_connection) = common::setup();

    // When
    let response = client
        .post("/category/update")
        .header(ContentType::Form)
        .body("id=123&name=foo")
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::NotFound);
}
