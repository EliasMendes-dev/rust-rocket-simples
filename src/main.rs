#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;

mod controllers;
mod database;
mod repositorios;
mod dtos;
mod models;
mod servicos;
mod utils;

use controllers::clientes_controller;
use controllers::home_controller;
use rocket::fs::{FileServer, relative};

#[launch]
async fn rocket() -> _ {
    let pool = database::conectar().await;

    rocket::build()
        .manage(pool)
        .mount(
            "/",
            routes![
                home_controller::index,
                clientes_controller::index,
                clientes_controller::novo,
                clientes_controller::criar,
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
