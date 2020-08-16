extern crate bigdecimal;
extern crate chrono;

use self::bigdecimal::BigDecimal;
use self::chrono::NaiveDateTime;
use common::schema::rezepte;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[table_name="rezepte"]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub ingredients: String,
    pub preparation: String,
    pub experience: Option<String>,
    pub time_need: Option<String>,
    pub number_people: Option<BigDecimal>,
    pub created: NaiveDateTime,
    pub owner: Option<BigDecimal>,
    pub rights: Option<BigDecimal>,
    pub category: Option<i32>
}

#[derive(Debug, Queryable, Serialize)]
pub struct RecipeName {
    pub id: i32,
    pub name: String,
}
