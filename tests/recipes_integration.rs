extern crate testcontainers;
extern crate rocket;
extern crate recipes_tool;
extern crate chrono;

use testcontainers::*;
use std::collections::HashMap;
use rocket::local::Client;
use rocket::http::Status;
use recipes_tool::init_application;
use std::env;
use chrono::Utc;
use recipes_tool::database::RecipeDatabase;
use recipes_tool::model::Recipe;

#[derive(Default)]
struct MySql;

impl Image for MySql {
    type Args = Vec<String>;
    type EnvVars = HashMap<String, String>;
    type Volumes = HashMap<String, String>;

    fn descriptor(&self) -> String {
        String::from("mysql:5")
    }

    fn wait_until_ready<D: Docker>(&self, container: &Container<D, Self>) {
        container
            .logs()
            .stderr
            .wait_for_message("port: 3306  MySQL Community Server (GPL)")
            .unwrap();
    }

    fn args(&self) -> <Self as Image>::Args {
        vec![]
    }

    fn env_vars(&self) -> Self::EnvVars {
        let mut env_vars = Self::EnvVars::new();
        env_vars.insert(String::from("MYSQL_USER"), String::from("rezepte"));
        env_vars.insert(String::from("MYSQL_PASSWORD"), String::from("rezepte-secret"));
        env_vars.insert(String::from("MYSQL_DATABASE"), String::from("rezepte"));
        env_vars.insert(String::from("MYSQL_RANDOM_ROOT_PASSWORD"), String::from("yes"));
        return env_vars;
    }

    fn volumes(&self) -> Self::Volumes {
        HashMap::new()
    }

    fn with_args(self, _arguments: <Self as Image>::Args) -> Self {
        self
    }
}

fn setup() -> (Client, RecipeDatabase) {
    // Start MySQL Docker container
    let docker = clients::Cli::default();
    let container = docker.run(MySql);
    let mysql_port = container.get_host_port(3306);
    let mysql_url = format!("{{recipe_db={{url=\"mysql://rezepte:rezepte-secret@127.0.0.1:{}/rezepte\"}}}}", mysql_port.unwrap());

    // Start Rocket application, which is under test
    env::set_var("ROCKET_DATABASES", mysql_url);
    let rocket = init_application();
    let database_connection = recipes_tool::database::RecipeDatabase::get_one(&rocket).unwrap();
    let client = Client::new(rocket).expect("valid rocket instance");

    (client, database_connection)
}

fn create_test_recipe() -> Recipe {
    Recipe {
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
        category: None
    }
}

#[test]
fn should_render_empty_recipe_list() {
    // Given
    let (client, _database_connection) = setup();

    // When
    let mut response = client.get("/").dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(!response.body_string().unwrap().contains("<li class=\"recipe\">"));
}

#[test]
fn should_render_recipe_list() {
    // Given
    let (client, database_connection) = setup();
    let recipe = create_test_recipe();
    recipes_tool::database::save_recipe(&recipe, database_connection);

    // When
    let mut response = client.get("/").dispatch();

    // Then
    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .body_string()
        .unwrap()
        .contains(&format!("<li class=\"recipe\"><a href=\"/recipe/{}\">{}</a></li>", recipe.id, recipe.name)));
}

#[test]
fn should_render_recipe() {
    // Given
    let (client, database_connection) = setup();
    let recipe = create_test_recipe();
    recipes_tool::database::save_recipe(&recipe, database_connection);

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
    let (client, _database_connection) = setup();

    // When
    let response = client.get("/recipe/22").dispatch();

    // Then
    assert_eq!(response.status(), Status::NotFound);
}
