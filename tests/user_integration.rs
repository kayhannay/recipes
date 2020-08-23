extern crate chrono;
extern crate recipes;
extern crate rocket;
extern crate testcontainers;

mod common;

use recipes::repository;
use rocket::http::{ContentType, Status};

#[test]
fn should_create_user() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .header(ContentType::Form)
        .body(format!(
            "username={}&password={}&name={}",
            user.username,
            user.password,
            user.name.clone().unwrap()
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let result = recipes::repository::user::get_user(&user.username, &database_connection);
    assert!(result.is_some());
    let result_user = result.unwrap();
    assert_eq!(result_user.username, user.username);
    assert_eq!(
        result_user.password,
        recipes::controller::common::create_hash(&user.password)
    );
    assert_eq!(result_user.name, user.name);
}

#[test]
fn should_not_create_user_no_username() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::NewRecipeUser {
        username: "".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .header(ContentType::Form)
        .body(format!(
            "username={}&password={}&name={}",
            user.username,
            user.password,
            user.name.clone().unwrap()
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not create user!"
    );
    let result = recipes::repository::user::get_user(&user.username, &database_connection);
    assert!(result.is_none());
}

#[test]
fn should_not_create_user_no_password() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .header(ContentType::Form)
        .body(format!(
            "username={}&password={}&name={}",
            user.username,
            user.password,
            user.name.clone().unwrap()
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not create user!"
    );
    let result = recipes::repository::user::get_user(&user.username, &database_connection);
    assert!(result.is_none());
}

#[test]
fn should_create_user_no_name() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "geheim".to_string(),
        name: None,
    };

    // When
    let response = client
        .post("/user")
        .header(ContentType::Form)
        .body(format!(
            "username={}&password={}&name=",
            user.username, user.password
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let result = recipes::repository::user::get_user(&user.username, &database_connection);
    assert!(result.is_some());
    let result_user = result.unwrap();
    assert_eq!(result_user.username, user.username);
    assert_eq!(
        result_user.password,
        recipes::controller::common::create_hash(&user.password)
    );
    assert_eq!(result_user.name, user.name);
}

#[test]
fn should_not_create_user_long_username() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::NewRecipeUser {
        username: "testusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestuser".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .header(ContentType::Form)
        .body(format!(
            "username={}&password={}&name={}",
            user.username,
            user.password,
            user.name.clone().unwrap()
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not create user!"
    );
    let result = recipes::repository::user::get_user(&user.username, &database_connection);
    assert!(result.is_none());
}

#[test]
fn should_not_create_user_exists() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };
    repository::user::save_user(&user, &database_connection).ok();

    // When
    let response = client
        .post("/user")
        .header(ContentType::Form)
        .body(format!(
            "username={}&password={}&name={}",
            user.username,
            user.password,
            user.name.clone().unwrap()
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not create user!"
    );
    let result = recipes::repository::user::get_user(&user.username, &database_connection);
    assert!(result.is_some());
}

#[test]
fn should_delete_user() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::NewRecipeUser {
        name: Some("Foo".to_string()),
        username: "foo".to_string(),
        password: "secret".to_string(),
    };
    recipes::repository::user::save_user(&user, &database_connection).ok();
    let user_id = recipes::repository::user::get_user(&user.username, &database_connection)
        .unwrap()
        .uid;

    // When
    let response = client.get(format!("/deleteuser/{}", user_id)).dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(flash_cookie.unwrap().value(), "7:successUser deleted");
    let result = recipes::repository::user::get_all_user(&database_connection);
    assert_eq!(result.len(), 0);
}

#[test]
fn should_not_delete_category() {
    // Given
    let (client, database_connection) = common::setup();

    // When
    let response = client.get(format!("/deleteuser/{}", 123)).dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not delete user!"
    );
    let result = recipes::repository::user::get_all_user(&database_connection);
    assert_eq!(result.len(), 0);
}
