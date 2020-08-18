extern crate chrono;
extern crate recipes;
extern crate rocket;
extern crate testcontainers;

mod common;

use rocket::http::Status;

#[test]
fn should_render_config_page() {
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
        .get("/config")
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .body_string()
        .unwrap()
        .contains("<h1>Rezepte: Konfiguration</h1>"));
}

#[test]
fn should_redirect_anonymous() {
    // Given
    let (client, _) = common::setup();

    // When
    let response = client.get("/config").dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}
