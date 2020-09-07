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
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .cookie(login_cookie.clone())
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
    let result = recipes::repository::user::get_user_by_name(&user.username, &database_connection);
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
fn should_require_authenticated_user_create_user() {
    // Given
    let (client, _database_connection) = common::setup();

    // When
    let response = client
        .post("/user")
        .header(ContentType::Form)
        .body("username=foo&password=bar&name=Foo")
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn should_not_create_user_no_username() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let user = recipes::domain::user::NewRecipeUser {
        username: "".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .cookie(login_cookie.clone())
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
    let result = recipes::repository::user::get_user_by_name(&user.username, &database_connection);
    assert!(result.is_none());
}

#[test]
fn should_not_create_user_no_password() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .cookie(login_cookie.clone())
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
    let result = recipes::repository::user::get_user_by_name(&user.username, &database_connection);
    assert!(result.is_none());
}

#[test]
fn should_create_user_no_name() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "geheim".to_string(),
        name: None,
    };

    // When
    let response = client
        .post("/user")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!(
            "username={}&password={}&name=",
            user.username, user.password
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let result = recipes::repository::user::get_user_by_name(&user.username, &database_connection);
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
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let user = recipes::domain::user::NewRecipeUser {
        username: "testusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestusertestuser".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };

    // When
    let response = client
        .post("/user")
        .cookie(login_cookie.clone())
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
    let result = recipes::repository::user::get_user_by_name(&user.username, &database_connection);
    assert!(result.is_none());
}

#[test]
fn should_not_create_user_exists() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let user = recipes::domain::user::NewRecipeUser {
        username: "testuser".to_string(),
        password: "geheim".to_string(),
        name: Some("Paul".to_string()),
    };
    repository::user::save_user(&user, &database_connection).ok();

    // When
    let response = client
        .post("/user")
        .cookie(login_cookie.clone())
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
    let result = recipes::repository::user::get_user_by_name(&user.username, &database_connection);
    assert!(result.is_some());
}

#[test]
fn should_delete_user() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);
    let user = recipes::domain::user::NewRecipeUser {
        name: Some("Foo".to_string()),
        username: "foo".to_string(),
        password: "secret".to_string(),
    };
    recipes::repository::user::save_user(&user, &database_connection).ok();
    let user_id = recipes::repository::user::get_user_by_name(&user.username, &database_connection)
        .unwrap()
        .id;

    // When
    let response = client
        .get(format!("/user/delete/{}", user_id))
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(flash_cookie.unwrap().value(), "7:successUser deleted");
    let result = recipes::repository::user::get_all_user(&database_connection);
    assert_eq!(result.len(), 1);
}

#[test]
fn should_not_delete_user() {
    // Given
    let (client, database_connection) = common::setup();
    let (_, login_cookie) = common::create_login_user(&client, &database_connection);

    // When
    let response = client
        .get(format!("/user/delete/{}", 123))
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let flash_cookie = common::get_cookie(&response, "_flash");
    assert_eq!(
        flash_cookie.unwrap().value(),
        "5:errorCould not delete user!"
    );
    let result = recipes::repository::user::get_all_user(&database_connection);
    assert_eq!(result.len(), 1);
}

#[test]
fn should_redirect_anonymous_delete() {
    // Given
    let (client, _) = common::setup();

    // When
    let response = client.get("/user/delete/123").dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn should_redirect_anonymous_update() {
    // Given
    let (client, _) = common::setup();

    // When
    let response = client.get("/user/update/123").dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/login"));
}

#[test]
fn should_render_update_user_form() {
    // Given
    let (client, database_connection) = common::setup();
    let (user, login_cookie) = common::create_login_user(&client, &database_connection);
    let user_id = recipes::repository::user::get_user_by_name(&user.username, &database_connection)
        .unwrap()
        .id;

    // When
    let mut response = client
        .get(format!("/user/update/{}", user_id))
        .cookie(login_cookie.clone())
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    let body = response.body_string().unwrap();
    assert!(body.contains("<title>Rezepte - Benutzer</title>"));
    assert!(body.contains(&format!("value=\"{}\"", user.username)));
    assert!(body.contains(&format!("value=\"{}\"", user_id)));
}

#[test]
fn should_update_user() {
    // Given
    let (client, database_connection) = common::setup();
    let (user, login_cookie) = common::create_login_user(&client, &database_connection);
    let user_id = recipes::repository::user::get_user_by_name(&user.username, &database_connection)
        .unwrap()
        .id;
    let updated_name = "Testuser";
    let updated_username = "updated-testuser";

    // When
    let response = client
        .post("/user/update")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!(
            "id={}&username={}&password=geheim&name={}",
            user_id, updated_username, updated_name
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let result_user = recipes::repository::user::get_user(user_id, &database_connection).unwrap();
    assert_eq!(result_user.username, updated_username);
    assert_eq!(result_user.password, user.password);
    assert_eq!(result_user.name, Some(updated_name.to_string()));
}

#[test]
fn should_require_authenticated_user_update_user() {
    // Given
    let (client, _database_connection) = common::setup();

    // When
    let response = client
        .post("/user/update")
        .header(ContentType::Form)
        .body("id=123&username=foo&password=bar&name=Foo")
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn should_update_user_password() {
    // Given
    let (client, database_connection) = common::setup();
    let (user, login_cookie) = common::create_login_user(&client, &database_connection);
    let user_id = recipes::repository::user::get_user_by_name(&user.username, &database_connection)
        .unwrap()
        .id;
    let updated_password = "updated_password";

    // When
    let response = client
        .post("/user/update")
        .cookie(login_cookie.clone())
        .header(ContentType::Form)
        .body(format!(
            "id={}&username={}&password={}&name=",
            user_id, user.username, updated_password,
        ))
        .dispatch();

    // Then
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some("/config"));
    let result_user =
        recipes::repository::user::get_user_by_name(&user.username, &database_connection).unwrap();
    assert_eq!(result_user.username, user.username);
    assert_eq!(
        result_user.password,
        recipes::controller::common::create_hash(&updated_password)
    );
    assert_eq!(result_user.name, user.name);
}
