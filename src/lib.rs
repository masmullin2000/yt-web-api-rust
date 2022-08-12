#![allow(non_snake_case)]

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use core::slice;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::TcpListener;

use crate::models::*;

pub mod models;

pub const AMT_OF_USERS: models::Int = 1000;

#[inline(always)]
pub fn make_user(idx: models::Int) -> User {
    let idx_str = idx.to_string();

    let mut f_name = String::new();
    f_name.push_str("FirstName");
    f_name.push_str(&idx_str);

    let mut l_name = String::new();
    l_name.push_str("LastName");
    l_name.push_str(&idx_str);

    User::new(idx, 25, f_name, l_name)
}

// pulled out of async fn users so that it can be benchmarked
pub fn get_users<'a>(amt: u16) -> &'a [User] {
    thread_local! {
        static USERS: RefCell<Vec<User>> = RefCell::new(Vec::new());
    }
    USERS.with(|u| {
        let users = &mut *u.borrow_mut();
        users.clear();

        for i in 1..=amt {
            let user = make_user(i);
            users.push(user);
        }

        let ptr = users.as_ptr();
        unsafe { slice::from_raw_parts(ptr, users.len()) }
    })
}

// moved out of async in order to run benchmarks
pub fn get_resp<'a>(amt: u16) -> &'a [u8] {
    thread_local! {
        static RESP: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    }

    RESP.with(|r| {
        let mut r_str = &mut *r.borrow_mut();
        r_str.clear();

        let users = get_users(amt);

        let writer = std::io::BufWriter::new(&mut r_str);
        serde_json::to_writer(writer, users).expect("could not serialize");

        let r_ptr = r_str.as_ptr();
        unsafe { slice::from_raw_parts(r_ptr, r_str.len()) }
    })
}

async fn users<'a>(req: actix_web::HttpRequest) -> &'a [u8] {
    let amt = if let Ok(params) = web::Query::<HashMap<String, u16>>::from_query(req.query_string())
    {
        if let Some(amt) = params.get("amt") {
            *amt
        } else {
            AMT_OF_USERS
        }
    } else {
        AMT_OF_USERS
    };

    get_resp(amt)
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
