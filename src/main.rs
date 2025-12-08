#[macro_use] extern crate rocket;
// Force recompile 1

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

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("Bibliotheque - Gestionnaire de livres personnel");
        println!("Usage: cargo run -- [OPTIONS]");
        println!();
        println!("Options:");
        println!("  --port, -p   Specify server port (default: 8000)");
        println!("  --create     Ensure database exists (runs migrations)");
        println!("  --recreate   Wipe and rebuild database (DESTRUCTIVE: deletes all data)");
        println!("  --purge      Delete database files and exit");
        println!("  --help, -h   Show this help message");
        std::process::exit(0);
    }

    if args.contains(&"--purge".to_string()) || args.contains(&"--recreate".to_string()) {
        println!("Removing database files...");
        if Path::new(db_path).exists() { let _ = fs::remove_file(db_path); }
        if Path::new(db_shm).exists() { let _ = fs::remove_file(db_shm); }
        if Path::new(db_wal).exists() { let _ = fs::remove_file(db_wal); }
        println!("Database removed.");
        
        if args.contains(&"--purge".to_string()) {
            std::process::exit(0);
        }
    }

    if args.contains(&"--create".to_string()) {
        println!("Ensuring database exists...");
    }

    let mut port_override = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--port" || args[i] == "-p" {
            if i + 1 < args.len() {
                if let Ok(p) = args[i+1].parse::<u16>() {
                    port_override = Some(p);
                } else {
                    eprintln!("Error: Invalid port number");
                    std::process::exit(1);
                }
            }
            i += 1;
        }
        i += 1;
    }

    let mut figment = rocket::Config::figment();
    if let Some(port) = port_override {
        figment = figment.merge(("port", port));
    }

    let mut app = rocket::custom(figment)
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
            routes::list_books,
            routes::edit_book_form,
            routes::edit_book,
            routes::get_metadata
        ])
        .mount("/uploads", rocket::fs::FileServer::from("uploads"));

    #[cfg(debug_assertions)]
    {
        app = app.mount("/", routes![routes::shutdown]);
    }
    
    app
}
