use repository::schema::categories;

#[derive(
    Debug, Queryable, Insertable, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd,
)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
}
