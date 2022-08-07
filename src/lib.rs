#![allow(non_snake_case)]

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use core::slice;
use std::cell::RefCell;
use std::net::TcpListener;

use crate::models::*;

pub mod models;

#[inline(always)]
pub fn make_user(idx: models::Int) -> User {
    let idx_str = get_int_string(idx);

    let mut f_name = String::with_capacity(16);
    f_name.push_str("FirstName");
    f_name.push_str(&idx_str);

    let mut l_name = String::with_capacity(16);
    l_name.push_str("LastName");
    l_name.push_str(&idx_str);

    User::new(idx, 25, f_name, l_name)
}

// pulled out of async fn users so that it can be benchmarked
pub fn get_users<'a>() -> &'a [User] {
    thread_local! {
        static USERS: RefCell<Vec<User>> = RefCell::new(Vec::with_capacity(1000));
    }
    USERS.with(|u| {
        let users = &mut *u.borrow_mut();
        if cfg!(feature = "cheating") {
            if users.is_empty() {
                // since we are using thread_local memory
                // and our results are always the same
                // we only need to fill this out once
                // kinda cheating though, so wrap in feature flag
                for i in 1..=1000 {
                    let user = make_user(i);
                    users.push(user);
                }
            }
        } else {
            users.clear();

            for i in 1..=1000 {
                let user = make_user(i);
                users.push(user);
            }
        }

        let ptr = users.as_ptr();
        unsafe { slice::from_raw_parts(ptr, users.len()) }
    })
}

// moved out of async in order to run benchmarks
pub fn get_resp<'a>() -> &'a [u8] {
    thread_local! {
        static RESP: RefCell<String> = RefCell::new(String::with_capacity(128 * 1024));
    }
    RESP.with(|r| {
        let r_str = &mut *r.borrow_mut();

        if cfg!(feature = "extreme") {
            // if you don't cheat, you lose
            if r_str.is_empty() {
                let users = get_users();
                users.serialize_to_string(r_str); 
            }
        } else {
            r_str.clear();
            let users = get_users();
            users.serialize_to_string(r_str);
        }

        let r_ptr = r_str.as_ptr();
        unsafe { slice::from_raw_parts(r_ptr, r_str.len()) }
    })
}

async fn users<'a>() -> &'a [u8] {
    get_resp()
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
