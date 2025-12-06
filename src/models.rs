use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;
use rocket::form::FromForm;

#[derive(Debug, Clone, FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub subtitle: Option<String>,
    pub author: String,
    pub year: Option<i64>,
    pub description: Option<String>,
    pub volume_number: Option<i64>,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BookForm {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: String,
    pub year: Option<i64>,
    pub description: Option<String>,
    pub volume_number: Option<i64>,
}
