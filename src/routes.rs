use rocket::{get, post, form::Form, response::Redirect, http::Status};
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx;

use crate::models::{Book, BookForm};
use crate::Db;


#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! { is_debug: cfg!(debug_assertions) })
}

#[get("/add")]
pub async fn add_book_form() -> Template {
    Template::render("add", context! { is_debug: cfg!(debug_assertions) })
}

#[post("/add", data = "<form>")]
pub async fn add_book(mut db: Connection<Db>, form: Form<BookForm>) -> Template {
    let book = form.into_inner();
    let result = sqlx::query(
        "INSERT INTO books (title, subtitle, author, year, description, series, volume_number) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&book.title)
    .bind(&book.subtitle)
    .bind(&book.author)
    .bind(book.year)
    .bind(&book.description)
    .bind(&book.series)
    .bind(book.volume_number)
    .execute(&mut **db)
    .await;

    match result {
        Ok(_) => Template::render("add", context! { 
            message: "Livre ajouté avec succès !",
            form: book,
            is_debug: cfg!(debug_assertions)
        }),
        Err(e) => Template::render("add", context! { 
            error: e.to_string(),
            form: book,
            is_debug: cfg!(debug_assertions)
        })
    }
}

// Bulk and Collection routes removed

#[get("/edit/<id>")]
pub async fn edit_book_form(mut db: Connection<Db>, id: i64) -> Result<Template, Status> {
    let book = sqlx::query_as::<_, Book>("SELECT id, title, subtitle, author, year, description, series, volume_number FROM books WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|_| Status::NotFound)?;

    Ok(Template::render("edit", context! { book: book, is_debug: cfg!(debug_assertions) }))
}

#[post("/edit/<id>", data = "<form>")]
pub async fn edit_book(mut db: Connection<Db>, id: i64, form: Form<BookForm>) -> Result<Redirect, Status> {
    let book = form.into_inner();
    sqlx::query("UPDATE books SET title = ?, subtitle = ?, author = ?, year = ?, description = ?, series = ?, volume_number = ? WHERE id = ?")
        .bind(book.title)
        .bind(book.subtitle)
        .bind(book.author)
        .bind(book.year)
        .bind(book.description)
        .bind(book.series)
        .bind(book.volume_number)
        .bind(id)
        .execute(&mut **db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Redirect::to("/list"))
}

#[get("/list")]
pub async fn list_books(mut db: Connection<Db>) -> Result<Template, String> {
    let books = sqlx::query_as::<_, Book>("SELECT id, title, subtitle, author, year, description, series, volume_number FROM books ORDER BY series ASC, volume_number ASC, year ASC, title ASC")
        .fetch_all(&mut **db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Template::render("list", context! { books: books, is_debug: cfg!(debug_assertions) }))
}

#[post("/shutdown")]
#[cfg(debug_assertions)]
pub fn shutdown(shutdown: rocket::Shutdown) {
    shutdown.notify();
}
