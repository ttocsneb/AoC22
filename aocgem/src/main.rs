use std::error::Error;

use cgi::{get_path, response};

use regex::Regex;

pub mod cgi;
pub mod fetch;
pub mod lead;
pub mod leaderboard;
pub mod nav;

fn handle() -> Result<(), Box<dyn Error>> {
    let paths: Vec<(Regex, &dyn Fn() -> Result<(), Box<dyn Error>>)> = vec![
        (Regex::new(r"^/[^/]+/\d{4}/[^/]+/?$")?, &lead::leaderboard),
        (
            Regex::new(r"^/[^/]+/\d{4}/]+/leaderboard/?$")?,
            &nav::leaderboard_select,
        ),
        (Regex::new(r"^/[^/]+/\d{4}/?$")?, &nav::leaderboard),
        (Regex::new(r"^/login/?$")?, &nav::login_request),
        (Regex::new(r"^/[^/]+/?$")?, &nav::year_select),
        (Regex::new(r"^/$")?, &nav::login),
    ];

    let path = get_path()?;

    for (rule, handler) in paths {
        if rule.is_match(&path) {
            return handler();
        }
    }

    Ok(response(51, "Path not found"))
}

fn main() {
    // handle().unwrap();
    if let Err(err) = handle() {
        response(42, err);
    }
}
