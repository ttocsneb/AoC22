use crate::{
    cgi::{get_query, get_script, OkResponse, Response, Result},
    fetch::load_pub_leaderboard,
    render::day::render_day,
};
use route_recognizer::{Params, Router};

use super::FnRoute;

fn view_session_day(params: &Params) -> Result<Response> {
    let session = params.find("session").unwrap();
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;
    let day = params.find("day").unwrap();
    let day = u32::from_str_radix(day, 10).ok_or_response(Response::not_found())?;

    let leaderboard = render_day(session, year, day, board_id)?;
    let script = get_script();

    let links = match day {
        1 => format!("=> {script}/session/{session}/{board_id}/{year}/2/ View day 2"),
        25 => format!("=> {script}/session/{session}/{board_id}/{year}/24/ View day 24"),
        _ => {
            let yesterday = day - 1;
            let tomorrow = day + 1;
            format!(
                "=> {script}/session/{session}/{board_id}/{year}/{yesterday}/ View day {yesterday}
=> {script}/session/{session}/{board_id}/{year}/{tomorrow}/ View day {tomorrow}"
            )
        }
    };

    Ok(Response::success(
        "text/gemini",
        format!(
            "
# Advent of Code Leaderboard {year}, Day {day}

=> {script}/session/{session}/{board_id}/{year}/ Back to the main leaderboard

Here are the times for Day {day}.

{leaderboard}

## View another day

{links}
"
        ),
    ))
}

fn view_public_day(params: &Params) -> Result<Response> {
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;
    let day = params.find("day").unwrap();
    let day = u32::from_str_radix(day, 10).ok_or_response(Response::not_found())?;

    let pub_board = load_pub_leaderboard(board_id).ok_or_response(Response::not_found())?;

    let leaderboard = render_day(&pub_board.session, year, day, &pub_board.id)?;
    let script = get_script();

    let links = match day {
        1 => format!("=> {script}/leaderboard/{board_id}/{year}/2/ View day 2"),
        25 => format!("=> {script}/leaderboard/{board_id}/{year}/24/ View day 24"),
        _ => {
            let yesterday = day - 1;
            let tomorrow = day + 1;
            format!(
                "=> {script}/leaderboard/{board_id}/{year}/{yesterday}/ View day {yesterday}
=> {script}/leaderboard/{board_id}/{year}/{tomorrow}/ View day {tomorrow}"
            )
        }
    };

    Ok(Response::success(
        "text/gemini",
        format!(
            "
# Advent of Code Leaderboard {year}, Day {day}

=> {script}/leaderboard/{board_id}/{year}/ Back to the leaderboard

Here are the times for Day {day}.

{leaderboard}

## View another day

{links}
"
        ),
    ))
}

fn select_day(params: &Params) -> Result<Response> {
    let session = params.find("session").unwrap();
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Which day would you like to view?"))
    } else {
        Ok(Response::redirect(format!(
            "{script}/session/{session}/{board_id}/{year}/{query}/"
        )))
    }
}

fn select_pub_day(params: &Params) -> Result<Response> {
    let board_id = params.find("leaderboard").unwrap();
    let year = params.find("year").unwrap();
    let year = i32::from_str_radix(year, 10).ok_or_response(Response::not_found())?;

    let script = get_script();
    let query = get_query();
    if query.is_empty() {
        Ok(Response::input("Which day would you like to view?"))
    } else {
        Ok(Response::redirect(format!(
            "{script}/leaderboard/{board_id}/{year}/{query}/"
        )))
    }
}

pub fn add_routes(router: &mut Router<&FnRoute>) {
    router.add(
        "/session/:session/:leaderboard/:year/:day",
        &view_session_day,
    );
    router.add("/session/:session/:leaderboard/:year/day", &select_day);
    router.add("/leaderboard/:leaderboard/:year/day", &select_pub_day);
    router.add("/leaderboard/:leaderboard/:year/:day", &view_public_day);
}
