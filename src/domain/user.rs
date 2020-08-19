use repository::schema::user;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[table_name = "user"]
pub struct RecipeUser {
    pub uid: i32,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[table_name = "user"]
pub struct NewRecipeUser {
    pub username: String,
    pub password: String,
    pub name: Option<String>,
}
