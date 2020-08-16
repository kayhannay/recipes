embed_migrations!();

#[database("recipe_db")]
pub struct RecipeDatabase(diesel::MysqlConnection);

pub fn run_migrations(connection: &diesel::MysqlConnection) {
    embedded_migrations::run_with_output(connection, &mut std::io::stdout()).expect("Could not run database migrations!");
}

