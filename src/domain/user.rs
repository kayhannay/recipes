use repository::schema::user;

#[derive(
    Debug, Queryable, Insertable, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd,
)]
#[table_name = "user"]
pub struct RecipeUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, AsChangeset, Identifiable)]
#[table_name = "user"]
pub struct UpdateRecipeUser {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[table_name = "user"]
pub struct NewRecipeUser {
    pub username: String,
    pub password: String,
    pub name: Option<String>,
}
