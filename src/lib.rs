#![allow(non_snake_case)]

use std::cell::RefCell;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::models::User;

mod models;

thread_local! {
    static USERS: RefCell<Vec<User>> = RefCell::new(Vec::with_capacity(1000));
}

fn make_user(idx: models::Int) -> User {
    User::new(idx, 25, format!("FirstName{idx}"), format!("LastName{idx}"))
}

pub fn get_users() {
    USERS.with(|u| {
        let users = &mut *u.borrow_mut();
        users.clear();
        for i in 1..=1000 {
            let user = make_user(i);
            users.push(user);
        }
    });
}

async fn users() -> HttpResponse {
    //let users = get_users();
    get_users();
    USERS.with(|u| {
        let users = &*u.borrow();
        HttpResponse::Ok().json(users)
    })
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || App::new().route("users", web::get().to(users)))
        // Setting the correct workers made a difference.
        .workers(num_cpus::get())
        // .workers(num_cpus::get_physical())
        .listen(listener)?
        .run();
    Ok(server)
}
