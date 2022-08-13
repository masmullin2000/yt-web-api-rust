#![allow(non_snake_case)]

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, HttpResponse};
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

    let mut f_name = models::StringType::from("FirstName");
    f_name.push_str(&idx_str);

    let mut l_name = models::StringType::from("LastName");
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

        // safety: thread_local immutable.  lifetime limited by 'a
        //         true lifetime is static
        //
        //         must NEVER escape to where an await can be called
        //
        // note: we can get rid of this unsafe call
        //       by having USERS be an Rc<RefCell<Vec<Users>>>
        //       and after filling users in the for loop
        //       doing a u.clone() as the return
        //
        //       this alternative is overly complex
        let ptr = users.as_ptr();
        unsafe { slice::from_raw_parts(ptr, users.len()) }
    })
}

pub fn get_resp(amt: u16) -> Vec<u8> {
    // note amount of bytes for a single User as Json formatted
    // is between 93 and 105 bytes.  128 is simply a nice binary number
    let mut resp = Vec::with_capacity((amt as usize) * 128);

    let users = get_users(amt);

    let writer = std::io::BufWriter::new(&mut resp);
    serde_json::to_writer(writer, users).expect("could not serialize");

    resp
}

async fn users(req: actix_web::HttpRequest) -> HttpResponse {
    let amt = if cfg!(feature = "query_string") {
        if let Ok(params) = web::Query::<HashMap<String, u16>>::from_query(req.query_string())
        {
            if let Some(amt) = params.get("amt") {
                *amt
            } else {
                AMT_OF_USERS
            }
        } else {
            AMT_OF_USERS
        }
    } else {
        AMT_OF_USERS
    };

    let resp = get_resp(amt);
    HttpResponse::Ok().body(resp)
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
