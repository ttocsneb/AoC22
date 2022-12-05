use crate::{
    cgi::{get_query, get_script, Response, Result},
    fetch::{
        fetch_leaderboard, find_pub_leaderboard, load_pub_leaderboard, pub_leaderboard_exists,
        save_pub_leaderboard,
    },
    leaderboard::PublicLeaderboard,
    render::render_leaderboard,
};
use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;
use route_recognizer::{Params, Router};

pub type FnRoute = dyn Fn(&Params) -> Result;

fn renew_pub_leaderboard(params: &Params) -> Result {
    // /leaderboard/:leaderboard/:year/renew/
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = match i32::from_str_radix(year, 10) {
        Ok(val) => val,
        Err(_) => return Ok(Response::NotFound),
    };

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

        let mut pub_board = match load_pub_leaderboard(board_id) {
            Ok(val) => val,
            Err(_) => return Ok(Response::NotFound),
        };

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

fn view_pub_leaderboard(params: &Params) -> Result {
    // /leaderboard/:leaderboard/:year/
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = match i32::from_str_radix(year, 10) {
        Ok(val) => val,
        Err(_) => return Ok(Response::NotFound),
    };

    let pub_board = match load_pub_leaderboard(board_id) {
        Ok(val) => val,
        Err(_) => return Ok(Response::NotFound),
    };

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

fn get_pub_year(params: &Params) -> Result {
    // /leaderboard/:leaderboard/
    let board_token = params.find("leaderboard").unwrap();

    if !pub_leaderboard_exists(board_token) {
        return Ok(Response::NotFound);
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

fn get_pub_leaderboard(_params: &Params) -> Result {
    // /leaderboard/

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Enter the leaderboard token"))
    } else {
        Ok(Response::redirect(format!("{script}/leaderboard/{query}/")))
    }
}

fn publish_leaderboard(params: &Params) -> Result {
    // /session/:session/:leaderboard/:year/publish/
    let session = params.find("session").unwrap();
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = match i32::from_str_radix(year, 10) {
        Ok(val) => val,
        Err(_) => return Ok(Response::NotFound),
    };

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

fn view_leaderboard(params: &Params) -> Result {
    // /session/:session/:leaderboard/:year/
    let session = params.find("session").unwrap();
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = match i32::from_str_radix(year, 10) {
        Ok(val) => val,
        Err(_) => return Ok(Response::NotFound),
    };

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

fn get_year(params: &Params) -> Result {
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

fn get_leaderboard(params: &Params) -> Result {
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

fn get_session(_params: &Params) -> Result {
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

fn root(_params: &Params) -> Result {
    // /
    let script = get_script();

    Ok(Response::success(
        "text/gemini",
        format!(
            r#"
# Advent of Code Leaderboard

Here you will be able to view a more detailed leaderboard than the one provided by the official Advent of Code site.

To open the capsule, you will need to get a token for the leaderboard you wish to view. This can be a leaderboard token that someone else has given you, or you can generate one yourself.

Public leaderboards are not advertised anywhere, so you will need to have a friend share the leaderboard token with you. If you have access to the leaderboard on adventofcode.com, you can view it here using the instructions below.

## I already have a leaderboard token

=> {script}/leaderboard/ Enter your leaderboard token here

## I don't have a leaderboard token

You can view a leaderboard without making it public. You will just need your session key and a leaderboard id. If you want to make the leaderboard public, there will be an option to publish it at the bottom of the main leaderboard.

### How to get your session key

When you log into adventofcode.com, it will save a cookie called 'session'. To find it, press F12 while on the site, navigate to the 'storage' or 'application' tab, then 'cookies', 'adventofcode.com' and you should see the session cookie. Copy the value for later.

=> https://adventofcode.com adventofcode.com

### How to get the leaderboard id

When you go to the leaderboard you wish to view, the last element of the url is the leaderboard id. Copy that for later.

```example
https://adventofcode.com/2022/leaderboard/private/view/123456
                                                       ^^^^^^
```

=> https://adventofcode.com/2022/leaderboard/private View your leaderboards

### Once you have your session key and leaderboard id

Follow the link below where you will be asked for your session key and leaderboard id. Once you enter them in, you will be able to view the leaderboard. If you want to make it public, you may do so from there

=> {script}/login/ Login to view your leaderboard

"#
        ),
    ))
}

pub fn add_routes(router: &mut Router<&FnRoute>) {
    router.add("/", &root);
    router.add("/session/", &get_session);
    router.add("/session/:session/", &get_leaderboard);
    router.add("/session/:session/:leaderboard/", &get_year);
    router.add(
        "/session/:session/:leaderboard/:year/publish/",
        &publish_leaderboard,
    );
    router.add("/session/:session/:leaderboard/:year/", &view_leaderboard);
    router.add("/leaderboard/", &get_pub_leaderboard);
    router.add("/leaderboard/:leaderboard/", &get_pub_year);
    router.add(
        "/leaderboard/:leaderboard/:year/renew/",
        &renew_pub_leaderboard,
    );
    router.add("/leaderboard/:leaderboard/:year/", &view_pub_leaderboard);
}
