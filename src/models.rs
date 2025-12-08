use rocket::serde::{Serialize};
use sqlx::FromRow;
use rocket::form::FromForm;
use rocket::fs::TempFile;

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
    pub cover_path: Option<String>,
}

#[derive(FromForm)]
pub struct BookForm<'r> {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: String,
    pub year: Option<i64>,
    pub description: Option<String>,
    pub volume_number: Option<i64>,
    pub cover: Option<TempFile<'r>>,
}
