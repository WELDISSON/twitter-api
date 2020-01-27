#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use(bson)] extern crate bson;

extern crate chrono;

mod lib;
mod meta;
mod models;
mod controllers;
mod utils;

fn main() {
    rocket::ignite().mount("/", routes![
        controllers::user::insert,
        controllers::user::get, 
        controllers::user::getAll, 
    ])
    .register(catchers![controllers::not_found::lookup])
    .launch();
}