use std::io::{self, Write};

use cgi::{get_path, Response, Result};

use route_recognizer::Router;
use routes::{add_routes, FnRoute};

pub mod cgi;
pub mod fetch;
pub mod leaderboard;
pub mod render;
pub mod routes;

fn handle() -> Result {
    let mut router = Router::<&FnRoute>::new();
    add_routes(&mut router);

    let path = get_path();

    let m = match router.recognize(&path) {
        Ok(val) => val,
        Err(_) => return Ok(Response::perm_error(format!("Could not find route for {path}"))),
    };
    let params = m.params();

    m.handler()(params)
}

fn main() {
    let response = std::panic::catch_unwind(|| match handle() {
        Ok(response) => response,
        Err(err) => {
            io::stderr().write_fmt(format_args!("{err}\n")).unwrap();
            Response::cgi_error(err)
        }
    });
    let response = match response {
        Ok(response) => response,
        Err(_) => Response::cgi_error("Internal Error"),
    };
    print!("{response}")
}
