use rocket::{get, post, form::Form, response::Redirect, http::Status, serde::json::Json};
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx;

use crate::models::{Book, BookForm};
use crate::Db;


use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::fs::TempFile;

// Helper to save file with unique name
async fn save_cover(mut file: TempFile<'_>) -> Option<String> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let filename = format!("cover_{}.jpg", timestamp);
    let path = Path::new("uploads").join(&filename);
    let temp_path = Path::new("uploads").join(format!("temp_{}", timestamp));
    
    // Ensure dir exists
    let _ = std::fs::create_dir_all("uploads");

    // First, copy the uploaded file to a temp location
    if let Err(e) = file.copy_to(&temp_path).await {
        eprintln!("Failed to copy temp file: {}", e);
        return None;
    }

    // Try to open and convert the image to JPEG
    match image::io::Reader::open(&temp_path) {
        Ok(reader) => {
            match reader.with_guessed_format() {
                Ok(reader_with_format) => {
                    // Check if format is supported
                    if let Some(format) = reader_with_format.format() {
                        eprintln!("Image format detected: {:?}", format);
                    }
                    
                    match reader_with_format.decode() {
                        Ok(img) => {
                            // Convert to RGB8 and save as JPEG
                            match img.to_rgb8().save_with_format(&path, image::ImageFormat::Jpeg) {
                                Ok(_) => {
                                    // Clean up temp file
                                    let _ = std::fs::remove_file(&temp_path);
                                    Some(format!("/uploads/{}", filename))
                                },
                                Err(e) => {
                                    eprintln!("Failed to save as JPEG: {}", e);
                                    let _ = std::fs::remove_file(&temp_path);
                                    None
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to decode image: {}", e);
                            let _ = std::fs::remove_file(&temp_path);
                            None
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to guess image format: {}", e);
                    let _ = std::fs::remove_file(&temp_path);
                    None
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to open image file: {}", e);
            let _ = std::fs::remove_file(&temp_path);
            None
        }
    }
}


#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! { is_debug: cfg!(debug_assertions) })
}

#[get("/add")]
pub async fn add_book_form() -> Template {
    Template::render("add", context! { is_debug: cfg!(debug_assertions) })
}

#[post("/add", data = "<form>")]
pub async fn add_book(mut db: Connection<Db>, mut form: Form<BookForm<'_>>) -> Template {
    eprintln!("DEBUG: Received form submission");
    
    let cover_path = if let Some(file) = form.cover.take() {
        eprintln!("DEBUG: File present, length: {}", file.len());
        if file.len() > 0 {
            let saved_path = save_cover(file).await;
            eprintln!("DEBUG: Saved path: {:?}", saved_path);
            saved_path
        } else {
            eprintln!("DEBUG: File length is 0");
            None
        }
    } else {
        eprintln!("DEBUG: No file in form");
        None
    };

    eprintln!("DEBUG: Final cover_path: {:?}", cover_path);

    let book = form.into_inner();
    let result = sqlx::query(
        "INSERT INTO books (title, subtitle, author, year, description, volume_number, cover_path) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&book.title)
    .bind(&book.subtitle)
    .bind(&book.author)
    .bind(book.year)
    .bind(&book.description)
    .bind(book.volume_number)
    .bind(&cover_path)
    .execute(&mut **db)
    .await;

    match result {
        Ok(_) => Template::render("add", context! { 
            message: "Livre ajouté avec succès !",
            is_debug: cfg!(debug_assertions)
        }),
        Err(e) => Template::render("add", context! { 
            error: e.to_string(),
            is_debug: cfg!(debug_assertions)
        })
    }
}

// Bulk and Collection routes removed

#[get("/edit/<id>")]
pub async fn edit_book_form(mut db: Connection<Db>, id: i64) -> Result<Template, Status> {
    let book = sqlx::query_as::<_, Book>("SELECT id, title, subtitle, author, year, description, volume_number, cover_path FROM books WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|_| Status::NotFound)?;

    Ok(Template::render("edit", context! { book: book, is_debug: cfg!(debug_assertions) }))
}

#[post("/edit/<id>", data = "<form>")]
pub async fn edit_book(mut db: Connection<Db>, id: i64, mut form: Form<BookForm<'_>>) -> Result<Redirect, Status> {
    // Handle cover image if provided
    let cover_path = if let Some(file) = form.cover.take() {
        if file.len() > 0 {
            save_cover(file).await
        } else {
            None
        }
    } else {
        None
    };

    let book = form.into_inner();
    
    // Update query - only update cover_path if a new image was uploaded
    if let Some(new_cover_path) = cover_path {
        sqlx::query("UPDATE books SET title = ?, subtitle = ?, author = ?, year = ?, description = ?, volume_number = ?, cover_path = ? WHERE id = ?")
            .bind(book.title)
            .bind(book.subtitle)
            .bind(book.author)
            .bind(book.year)
            .bind(book.description)
            .bind(book.volume_number)
            .bind(new_cover_path)
            .bind(id)
            .execute(&mut **db)
            .await
            .map_err(|_| Status::InternalServerError)?;
    } else {
        sqlx::query("UPDATE books SET title = ?, subtitle = ?, author = ?, year = ?, description = ?, volume_number = ? WHERE id = ?")
            .bind(book.title)
            .bind(book.subtitle)
            .bind(book.author)
            .bind(book.year)
            .bind(book.description)
            .bind(book.volume_number)
            .bind(id)
            .execute(&mut **db)
            .await
            .map_err(|_| Status::InternalServerError)?;
    }

    Ok(Redirect::to("/list"))
}

#[get("/list?<q>&<sort>")]
pub async fn list_books(mut db: Connection<Db>, q: Option<String>, sort: Option<String>) -> Result<Template, String> {
    let mut query_str = String::from("SELECT id, title, subtitle, author, year, description, volume_number, cover_path FROM books");
    
    if let Some(ref search) = q {
        if !search.is_empty() {
             query_str.push_str(" WHERE title LIKE '%");
             query_str.push_str(search);
             query_str.push_str("%' OR author LIKE '%");
             query_str.push_str(search);
             query_str.push_str("%'");
        }
    }

    let order = match sort.as_deref() {
        Some("author") => " ORDER BY author ASC, title ASC",
        Some("title") | _ => " ORDER BY title ASC, volume_number ASC, year ASC",
    };
    query_str.push_str(order);

    let books = sqlx::query_as::<_, Book>(&query_str)
        .fetch_all(&mut **db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Template::render("list", context! { 
        books: books, 
        is_debug: cfg!(debug_assertions),
        search_query: q,
        current_sort: sort.unwrap_or_else(|| "title".to_string())
    }))
}

#[get("/api/books/metadata")]
pub async fn get_metadata(mut db: Connection<Db>) -> Json<Vec<Book>> {
    let books = sqlx::query_as::<_, Book>("SELECT id, title, subtitle, author, year, description, volume_number, cover_path FROM books")
        .fetch_all(&mut **db)
        .await
        .unwrap_or_default();
    
    Json(books)
}

#[post("/shutdown")]
#[cfg(debug_assertions)]
pub fn shutdown(shutdown: rocket::Shutdown) {
    shutdown.notify();
}
