#[macro_use] extern crate rocket;

mod models;
mod routes;

use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::Template;

#[derive(Database)]
#[database("bibliotheque")]
pub struct Db(sqlx::SqlitePool);

use rocket::fairing::AdHoc;

use std::env;
use std::fs;
use std::path::Path;

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = env::args().collect();
    let db_path = "bibliotheque.db";
    let db_shm = "bibliotheque.db-shm";
    let db_wal = "bibliotheque.db-wal";

    if args.contains(&"--purge".to_string()) || args.contains(&"--recreate".to_string()) {
        println!("Removing database files...");
        let _ = fs::remove_file(db_path);
        let _ = fs::remove_file(db_shm);
        let _ = fs::remove_file(db_wal);
        println!("Database removed.");
        
        if args.contains(&"--purge".to_string()) {
            std::process::exit(0);
        }
    }

    if args.contains(&"--create".to_string()) {
        println!("Ensuring database exists...");
    }

    let mut app = rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", |rocket| async {
            let Some(db) = Db::fetch(&rocket) else {
                error!("Failed to connect to database");
                return Err(rocket);
            };

            match sqlx::migrate!().run(&**db).await {
                Ok(_) => Ok(rocket),
                Err(e) => {
                    error!("Failed to run migrations: {}", e);
                    Err(rocket)
                }
            }
        }))
        .attach(Template::fairing())
        .mount("/", routes![
            routes::index,
            routes::add_book_form,
            routes::add_book,
            routes::index,
            routes::add_book_form,
            routes::add_book,
            routes::list_books,
            routes::edit_book_form,
            routes::edit_book
        ]);

    #[cfg(debug_assertions)]
    {
        app = app.mount("/", routes![routes::shutdown]);
    }
    
    app
}
