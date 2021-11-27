use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::models::User;

mod models;

fn get_users() -> Vec<User> {
    let mut users = Vec::with_capacity(1000);
    for index in 1..1001_u16 {
        users.push(User {
            id: index,
            age: 25,
            first_name: format!("firstName{}", index),
            last_name: format!("lastName{}", index),
            framework: "Rust (actix-web)".to_owned(),
        })
    }
    users
}

pub async fn users() -> HttpResponse {
    HttpResponse::Ok().json(get_users())
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new().service(web::scope("/api/v1").route("users", web::get().to(users)))
    })
    .listen(listener)?
    .run();
    Ok(server)
}