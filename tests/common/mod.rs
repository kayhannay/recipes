use rocket::http::{ContentType, Cookie};
use rocket::local::{Client, LocalResponse};
use std::collections::HashMap;
use std::env;
use testcontainers::*;

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
        env_vars.insert(String::from("MYSQL_USER"), String::from("recipes"));
        env_vars.insert(
            String::from("MYSQL_PASSWORD"),
            String::from("recipes-secret"),
        );
        env_vars.insert(String::from("MYSQL_DATABASE"), String::from("recipes"));
        env_vars.insert(
            String::from("MYSQL_RANDOM_ROOT_PASSWORD"),
            String::from("yes"),
        );
        return env_vars;
    }

    fn volumes(&self) -> Self::Volumes {
        HashMap::new()
    }

    fn with_args(self, _arguments: <Self as Image>::Args) -> Self {
        self
    }
}

pub fn setup() -> (Client, recipes::repository::common::RecipeDatabase) {
    // Start MySQL Docker container
    let docker = clients::Cli::default();
    let container = docker.run(MySql);
    let mysql_port = container.get_host_port(3306).unwrap();
    println!("Test database port: {}", mysql_port);
    let mysql_url = format!(
        "{{recipe_db={{url=\"mysql://recipes:recipes-secret@127.0.0.1:{}/recipes\"}}}}",
        mysql_port
    );

    // Start Rocket application, which is under test
    env::set_var("ROCKET_DATABASES", mysql_url);
    let rocket = recipes::init_application();
    let database_connection =
        recipes::repository::common::RecipeDatabase::get_one(&rocket).unwrap();
    let client = Client::new(rocket).expect("valid rocket instance");

    (client, database_connection)
}

pub fn get_cookie(response: &LocalResponse<'_>, name: &str) -> Option<Cookie<'static>> {
    let cookie = response
        .headers()
        .get("Set-Cookie")
        .filter(|v| v.starts_with(name))
        .nth(0)
        .and_then(|val| Cookie::parse_encoded(val).ok());

    cookie.map(|c| c.into_owned())
}

pub fn create_login_user(
    client: &Client,
    database_connection: &recipes::repository::common::RecipeDatabase,
) -> (recipes::domain::user::NewRecipeUser, Cookie<'static>) {
    let password = "geheim";
    let user = recipes::domain::user::NewRecipeUser {
        username: "loginuser".to_string(),
        password: recipes::controller::common::create_hash(password),
        name: None,
    };
    recipes::repository::user::save_user(&user, &database_connection).ok();
    let login_cookie = login(&client, &user.username, password).expect("logged in");
    (user, login_cookie)
}

pub fn login(client: &Client, user: &str, pass: &str) -> Option<Cookie<'static>> {
    let response = client
        .post("/login")
        .header(ContentType::Form)
        .body(format!("username={}&password={}", user, pass))
        .dispatch();

    get_cookie(&response, recipes::controller::common::COOKIE_NAME)
}
