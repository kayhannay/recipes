extern crate bigdecimal;
extern crate chrono;

use self::bigdecimal::BigDecimal;
use self::chrono::NaiveDateTime;

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct Recipe {
    id: i32,
    name: String,
    pub ingredients: String,
    pub preparation: String,
    experience: Option<String>,
    time_needed: Option<String>,
    number_people: Option<BigDecimal>,
    created: NaiveDateTime,
    owner: Option<BigDecimal>,
    rights: Option<BigDecimal>,
    category: Option<i32>
}

#[derive(Debug, Queryable, Serialize)]
pub struct RecipeName {
    id: i32,
    name: String,
}
