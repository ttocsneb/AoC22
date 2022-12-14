use crate::{
    cgi::{get_query, get_script, OkResponse, Response, Result},
    fetch::{
        fetch_leaderboard, find_pub_leaderboard, load_pub_leaderboard, pub_leaderboard_exists,
        save_pub_leaderboard,
    },
    leaderboard::PublicLeaderboard,
    render::main::render_leaderboard,
};
use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;
use route_recognizer::{Params, Router};
use urlencoding::decode;

use super::FnRoute;

fn renew_pub_leaderboard(params: &Params) -> Result<Response> {
    // /leaderboard/:leaderboard/:year/renew/
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;

    let script = get_script();
    let query = get_query();
    let query = decode(&query).ok_or_response(Response::bad_request())?;
    if query.is_empty() {
        Ok(Response::input("Enter your session key"))
    } else {
        // Account for copying from firefox (session:"{session}")
        let regex = Regex::new(r#""(.*)""#).unwrap();
        let session = if let Some(captures) = regex.captures(&query) {
            captures.get(1).unwrap().as_str()
        } else {
            &query
        };

        let mut pub_board = load_pub_leaderboard(board_id).ok_or_response(Response::not_found())?;

        // Make sure the session is valid
        if let Err(_) = fetch_leaderboard(&session, &pub_board.id, year) {
            return Ok(Response::perm_error("You must provide a valid session"));
        }

        pub_board.session = session.to_owned();

        save_pub_leaderboard(&board_id, &pub_board).unwrap();

        Ok(Response::redirect(format!(
            "{script}/leaderboard/{board_id}/{year}/"
        )))
    }
}

fn view_pub_leaderboard(params: &Params) -> Result<Response> {
    // /leaderboard/:leaderboard/:year/
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;

    let pub_board = load_pub_leaderboard(board_id).ok_or_response(Response::not_found())?;

    let script = get_script();
    match render_leaderboard(&pub_board.session, year, &pub_board.id) {
        Ok(leaderboard) => Ok(Response::success(
            "text/gemini",
            format!(
                "
# Advent of Code Leaderboard {year}

=> {script}/leaderboard/{board_id}/ View a different year

> This is a public leaderboard, the leaderboard token is `{board_id}`

The overall scores for each person in the leaderboard.

{leaderboard}
"
            ),
        )),
        Err(_) => {
            // The session is invalid
            Ok(Response::success("text/gemini", format!("
# Session has expired 

The session for this leaderboard has expired. To renew the leaderboard, a new session key will need to be provided. If you have access to the leaderboard, you can renew session with your own session key.

=> {script}/leaderboard/{board_id}/{year}/renew/ Renew the leaderboard

### How to get your session key

When you log into adventofcode.com, it will save a cookie called 'session'. To find it, press F12 while on the site, navigate to the 'storage' or 'application' tab, then 'cookies', 'adventofcode.com' and you should see the session cookie. Copy the value for later.

=> https://adventofcode.com adventofcode.com

")))
        }
    }
}

fn get_pub_year(params: &Params) -> Result<Response> {
    // /leaderboard/:leaderboard/
    let board_token = params.find("leaderboard").unwrap();

    if !pub_leaderboard_exists(board_token) {
        return Ok(Response::not_found());
    }

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Which year would you like to view?"))
    } else {
        Ok(Response::redirect(format!(
            "{script}/leaderboard/{board_token}/{query}/"
        )))
    }
}

fn get_pub_leaderboard(_params: &Params) -> Result<Response> {
    // /leaderboard/

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Enter the leaderboard token"))
    } else {
        Ok(Response::redirect(format!("{script}/leaderboard/{query}/")))
    }
}

fn publish_leaderboard(params: &Params) -> Result<Response> {
    // /session/:session/:leaderboard/:year/publish/
    let session = params.find("session").unwrap();
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;

    let query = get_query();
    if query.is_empty() {
        Ok(Response::input(
            "Are you sure you want to publish this leaderboard? (enter `yes`)",
        ))
    } else {
        let script = get_script();
        if query.to_lowercase() != "yes" {
            Ok(Response::redirect(format!(
                "{script}/session/{session}/{board_id}/{year}/"
            )))
        } else {
            // Make sure the session is valid
            if let Err(_) = fetch_leaderboard(&session, &board_id, year) {
                return Ok(Response::perm_error(
                    "You must provide a valid session/leaderboard id",
                ));
            }

            if let Some(mut pub_board) = find_pub_leaderboard(board_id)? {
                pub_board.session = session.into();
                let token = &pub_board.token;
                save_pub_leaderboard(&token, &pub_board)?;

                return Ok(Response::redirect(format!(
                    "{script}/leaderboard/{token}/{year}/"
                )));
            }

            loop {
                let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
                if !pub_leaderboard_exists(&token) {
                    let leaderboard = PublicLeaderboard::new(&token, board_id, session);
                    save_pub_leaderboard(&token, &leaderboard)?;
                    return Ok(Response::redirect(format!(
                        "{script}/leaderboard/{token}/{year}/"
                    )));
                }
            }
        }
    }
}

fn view_leaderboard(params: &Params) -> Result<Response> {
    // /session/:session/:leaderboard/:year/
    let session = params.find("session").unwrap();
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;

    let leaderboard = render_leaderboard(session, year, board_id)?;
    let script = get_script();

    Ok(Response::success(
        "text/gemini",
        format!(
            "
# Advent of Code Leaderboard {year}

=> {script}/session/{session}/{board_id}/ View a different year

The overall scores for each person in the leaderboard.

{leaderboard}

=> {script}/session/{session}/{board_id}/{year}/publish/ Make this leaderboard public
"
        ),
    ))
}

fn get_year(params: &Params) -> Result<Response> {
    // /session/:session/:leaderboard/
    let session = params.find("session").unwrap();
    let leaderboard = params.find("leaderboard").unwrap();

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Which year would you like to view?"))
    } else {
        Ok(Response::redirect(format!(
            "{script}/session/{session}/{leaderboard}/{query}/"
        )))
    }
}

fn get_leaderboard(params: &Params) -> Result<Response> {
    // /session/:session/
    let session = params.find("session").unwrap();

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Enter your leaderboard id"))
    } else {
        Ok(Response::redirect(format!(
            "{script}/session/{session}/{query}/"
        )))
    }
}

fn get_session(_params: &Params) -> Result<Response> {
    // /session/

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Enter your session key"))
    } else {
        // Account for copying from firefox (session:"{session}")
        let regex = Regex::new(r#""(.*)""#).unwrap();
        let session = if let Some(captures) = regex.captures(&query) {
            captures.get(1).unwrap().as_str()
        } else {
            &query
        };
        Ok(Response::redirect(format!("{script}/session/{session}/")))
    }
}

pub fn add_routes(router: &mut Router<&FnRoute>) {
    router.add("/session", &get_session);
    router.add("/session/:session", &get_leaderboard);
    router.add("/session/:session/:leaderboard", &get_year);
    router.add(
        "/session/:session/:leaderboard/:year/publish",
        &publish_leaderboard,
    );
    router.add("/session/:session/:leaderboard/:year", &view_leaderboard);
    router.add("/leaderboard", &get_pub_leaderboard);
    router.add("/leaderboard/:leaderboard", &get_pub_year);
    router.add(
        "/leaderboard/:leaderboard/:year/renew",
        &renew_pub_leaderboard,
    );
    router.add("/leaderboard/:leaderboard/:year", &view_pub_leaderboard);
}
