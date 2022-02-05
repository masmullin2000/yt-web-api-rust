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
    // let users = actix_web::web::block(|| get_users()).await.unwrap();
    let users = tokio::task::spawn_blocking(|| get_users()).await.unwrap();
    HttpResponse::Ok().json(users)
}

fn main() -> std::io::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        // let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .worker_threads(num_cpus::get_physical())
        // .worker_threads(num_cpus::get())
        .build()
        .unwrap();

    runtime.block_on(async {
        let _ = HttpServer::new(move || App::new().route("users", web::get().to(users)))
            .workers(num_cpus::get_physical())
            .bind("0.0.0.0:8083")
            .unwrap()
            .run()
            .await;
    });

    Ok(())
}
