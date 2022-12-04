use std::error::Error;

use crate::cgi::{get_path, get_query, get_script, response, success};

/// /
pub fn login() -> Result<(), Box<dyn Error>> {
    Ok(success(
        "text/gemini",
        "
# Advent Of Code Leaderboard

Here you will be able to view a more advanced leaderboard than the one provided by advent of code. 

## You need to provide a session key

In order to use this page, you will need to provide your session key from

=> https://adventofcode.com adventofcode.com

The easiest way to access your session key would be to go to adventofcode.com, press F12 to access the inspector, navigate to the 'storage' tab, 'cookies', and copy the value for the 'session' cookie.

Once you have your cookie, you may log in using the link below.

=> login Enter your session key

"
    ))
}

/// /login
pub fn login_request() -> Result<(), Box<dyn Error>> {
    let session = get_query()?;
    if session.is_empty() {
        response(10, "Paste your session key here");
        return Ok(());
    }

    let script = get_script()?;
    Ok(response(30, format!("{script}/{session}/")))
}

/// /{session}/
pub fn year_select() -> Result<(), Box<dyn Error>> {
    let year = get_query()?;
    if year.is_empty() {
        Ok(response(10, "Which year would you like to view"))
    } else {
        let script = get_script()?;
        let path = get_path()?;
        let session = path.split('/').nth(1).unwrap();
        Ok(response(30, format!("{script}/{session}/{year}/")))
    }
}

/// /{session}/{year}/
pub fn leaderboard() -> Result<(), Box<dyn Error>> {
    let path = get_path()?;
    let year = path.split('/').nth(2).unwrap();
    Ok(success("text/gemini", &format!("
# Advent Of Code Leaderboard

Here you will be able to view a more advanced leaderboard than the one provided by advent of code. 

## You need to provide a leaderboard id

In order to use this capsule, you will need to provide the leaderboard id that you would like to view.

You can get the leaderboard id by going to the official leaderboard. In the url, the last segment is the id:

```example
https://adventofcode.com/{year}/leaderboard/private/view/123456
                                                       ^^^^^^
```

=> https://adventofcode.com/{year}/leaderboard/private View a list of your leaderboards on adventofcode.com


Once you have your cookie, you may log in using the link below.

=> leaderboard Enter your leaderboard id

")))
}

/// /{session}/{year}/leaderboard
pub fn leaderboard_select() -> Result<(), Box<dyn Error>> {
    let leaderboard = get_query()?;
    if leaderboard.is_empty() {
        Ok(response(
            10,
            "Paste the leaderboard id you would like to view",
        ))
    } else {
        let script = get_script()?;
        let path = get_path()?;
        let mut path = path.split('/');
        let session = path.nth(1).unwrap();
        let year = path.next().unwrap();
        Ok(response(
            30,
            format!("{script}/{session}/{year}/{leaderboard}/"),
        ))
    }
}
