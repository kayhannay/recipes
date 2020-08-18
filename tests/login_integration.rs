extern crate rocket;
extern crate testcontainers;

mod common;

use rocket::http::{ContentType, Status};

#[test]
fn should_render_login_form() {
    // Given
    let (client, _) = common::setup();

    // When
    let mut response = client.get("/login").dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .body_string()
        .unwrap()
        .contains("<title>Rezepte - Login</title>"));
}

#[test]
fn should_login_by_cookie() {
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
    let response = client.get("/login").cookie(login_cookie.clone()).dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));
}

#[test]
fn should_login() {
    // Given
    let (client, database_connection) = common::setup();
    let password = "geheim";
    let user = recipes::domain::user::RecipeUser {
        username: "testuser".to_string(),
        password: recipes::controller::common::create_hash(password),
        name: None,
    };
    recipes::repository::user::save_user(&user, &database_connection).ok();

    // When
    let response = client
        .post("/login")
        .header(ContentType::Form)
        .body(format!("username={}&password={}", user.username, password))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/"));
}

#[test]
fn should_fail_login() {
    // Given
    let (client, database_connection) = common::setup();
    let user = recipes::domain::user::RecipeUser {
        username: "testuser".to_string(),
        password: recipes::controller::common::create_hash("geheim"),
        name: None,
    };
    recipes::repository::user::save_user(&user, &database_connection).ok();

    // When
    let response = client
        .post("/login")
        .header(ContentType::Form)
        .body(format!("username={}&password={}", user.username, "wrong"))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn should_logout() {
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
    let response = client
        .post("/logout")
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "7:successSuccessfully logged out."
    );
}
