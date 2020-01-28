#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]
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
    rocket::ignite().mount("/api/", routes![
        controllers::user::insert,
        controllers::user::getAll,
        controllers::tweet::get, 
        controllers::tweet::getAll, 
        controllers::tweet::insert,
        controllers::tweet::getAllFromUser, 
        controllers::tweet::like, 
        controllers::tweet::retweet
    ])
    .register(catchers![controllers::not_found::lookup])
    .launch();
}