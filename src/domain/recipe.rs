extern crate bigdecimal;
extern crate chrono;

use self::bigdecimal::BigDecimal;
use self::chrono::NaiveDateTime;
use repository::schema::rezepte;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[table_name = "rezepte"]
pub struct Recipe {
    pub name: String,
    pub ingredients: String,
    pub preparation: String,
    pub category: Option<i32>,
    pub number_people: Option<BigDecimal>,
    pub experience: Option<String>,
    pub created: NaiveDateTime,
    pub rights: Option<BigDecimal>,
    pub owner: Option<BigDecimal>,
    pub time_need: Option<String>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct RecipeName {
    pub id: i32,
    pub name: String,
}
