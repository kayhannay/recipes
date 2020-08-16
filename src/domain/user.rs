use repository::schema::user;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[table_name = "user"]
pub struct RecipeUser {
    pub username: String,
    pub password: String,
    pub name: Option<String>,
}
