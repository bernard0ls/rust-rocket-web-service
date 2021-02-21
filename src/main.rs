#![feature(decl_macro)]
#[macro_use] extern crate rocket;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;

#[database("rocket_web_service_db")]
pub struct DbConn(diesel::PgConnection);

#[get("/health")]
fn healthcheck() -> String {
    format!("OK")
}

fn main() {
    rocket::ignite()
    .mount("/", routes![
        routes::post::get_posts,
        routes::post::create_post
    ])
    .mount("/healthcheck", routes![healthcheck])
    .attach(DbConn::fairing())
    .attach(cors::CorsFairing)
    .launch();
}
