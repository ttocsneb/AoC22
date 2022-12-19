pub mod day;
pub mod main;

use crate::cgi::{get_script, Response, Result};
use route_recognizer::{Params, Router};

pub type FnRoute = dyn Fn(&Params) -> Result<Response>;

fn root(_params: &Params) -> Result<Response> {
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

=> {script}/session/ Login to view your leaderboard

"#
        ),
    ))
}

pub fn add_routes(router: &mut Router<&FnRoute>) {
    router.add("/", &root);
    main::add_routes(router);
    day::add_routes(router);
}
