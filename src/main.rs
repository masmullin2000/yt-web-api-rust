use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::models::User;

mod models;

fn get_users() -> Vec<User> {
    let mut users = Vec::with_capacity(1000);
    for index in 1..1001_u16 {
        users.push(User {
            Id: index,
            Age: 25,
            First_Name: format!("First_Name{}", index),
            Last_Name: format!("Last_Name{}", index),
            Framework: "Rust (actix-web)".to_owned(),
        })
    }
    users
}

async fn users() -> HttpResponse {
    let users = tokio::task::spawn_blocking(|| get_users()).await.unwrap();
    HttpResponse::Ok().json(users)
}

fn main() -> std::io::Result<()> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let listener =
            std::net::TcpListener::bind("0.0.0.0:8083").expect("Failed to bind to port 80");
        let server = HttpServer::new(move || App::new().route("users", web::get().to(users)))
            // Setting the correct workers made a difference.
            // .workers(num_cpus::get_physical())
            .listen(listener)
            .unwrap()
            .run()
            .await;
    });

    Ok(())
}
