#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;
mod repositories;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use repositories::RustaceanRepository;
use auth::BasicAuth;
use models::{Rustacean, NewRustacean};
use rocket::serde::json::{Value, json, Json};
use rocket::response::status::{self, Custom};
use core::panic;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustaceans(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustaceans(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(id: i32, _auth: BasicAuth, db: DbConn, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id: i32, _auth: BasicAuth, db: DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
       RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found, Man!")
}

async fn run_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>>  {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");
    DbConn::get_one(&rocket).await
        .expect("---- 333 Failed database connection ----")
        .run(|conn| match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => {
                
            },
            Err(e) => {
                panic!("faild conn {e:?}");
            }
         })
        .await;
    Ok(rocket)
}

#[rocket::main]
async fn main() {

    let _ = rocket::build()
    .attach(DbConn::fairing())
    .attach(AdHoc::try_on_ignite("Running DB Migrations", run_migrations))
    .mount("/", routes![
        get_rustaceans,
        view_rustaceans,
        create_rustaceans,
        update_rustaceans,
        delete_rustaceans
    ])
    .register("/", catchers![
        not_found
    ])
    .launch()
    .await;
}
