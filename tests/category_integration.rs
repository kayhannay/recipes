extern crate chrono;
extern crate rocket;
extern crate testcontainers;

mod common;

use rocket::http::{ContentType, Status};

#[test]
fn should_create_category() {
    // Given
    let (client, database_connection) = common::setup();
    let category = recipes::domain::category::NewCategory {
        name: "Category1".to_string(),
    };

    // When
    let response = client
        .post("/category")
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
fn should_not_create_category() {
    // Given
    let (client, database_connection) = common::setup();
    let category = recipes::domain::category::NewCategory {
        name: "Category1".to_string(),
    };
    recipes::repository::category::save_category(&category, &database_connection).ok();

    // When
    let response = client
        .post("/category")
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