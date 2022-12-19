use std::cmp::Ordering;

use chrono::Duration;

use crate::cgi::{Error, Response, Result};

use crate::leaderboard::Leaderboard;
use crate::{cgi::parse_query, fetch::get_leaderboard};

use super::render_duration;

pub fn render_table(
    leaderboard: &Leaderboard,
    sort_method: &str,
    year: i32,
    day: u32,
) -> Result<String> {
    let mut members = Vec::new();

    for (_, member) in &leaderboard.members {
        let (part1, part2) = member.completion_time(day, year);
        let total = match part1 {
            Some(t) => Some(
                t + match part2 {
                    Some(t) => t,
                    None => Duration::zero(),
                },
            ),
            None => None,
        };
        members.push((
            // Sorting elements
            total,
            part1,
            part2,
            // Data
            &member.name,
            match &part1 {
                Some(d) => render_duration(d),
                None => "--:--:--".to_owned(),
            },
            match &part2 {
                Some(d) => render_duration(d),
                None => "--:--:--".to_owned(),
            },
            match &total {
                Some(d) => render_duration(d),
                None => "--:--:--".to_owned(),
            },
        ));
    }

    fn compare_time(a: Option<Duration>, b: Option<Duration>) -> Ordering {
        if let Some(a) = a {
            if let Some(b) = b {
                a.cmp(&b)
            } else {
                Ordering::Less
            }
        } else {
            if let Some(_) = b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }

    match sort_method {
        "part1" => members.sort_unstable_by(|a, b| match compare_time(a.1, b.1) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => compare_time(a.2, b.2),
        }),
        "part2" => members.sort_unstable_by(|a, b| match compare_time(a.2, b.2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => compare_time(a.1, b.1),
        }),
        _ => members.sort_unstable_by(|a, b| compare_time(a.0, b.0)),
    }

    let mut part1_w = 0;
    let mut part2_w = 0;
    let mut total_w = 0;
    let mut name_w = 0;
    for (_, _, _, name, part1, part2, total) in &members {
        part1_w = part1.len().max(part1_w);
        part2_w = part2.len().max(part2_w);
        total_w = total.len().max(total_w);
        name_w = name.len().max(name_w);
    }

    let mut buffer = String::new();

    let n_t = "";
    let name_t = "Name";
    let part1_t = "Part 1";
    let part2_t = "Part 2";
    let total_t = "Total";

    let n_w = members.len().to_string().len() + 1;

    buffer += &format!(
        "{n_t:<n_w$} {total_t:<total_w$} {part1_t:<part1_w$} {part2_t:<part2_w$} {name_t:<name_w$}"
    );

    for (i, (_, _, _, name, part1, part2, total)) in members.into_iter().enumerate() {
        let i = i + 1;
        let i = format!("{i}.");
        buffer += &format!(
            "\n{i:>n_w$} {total:>total_w$} {part1:>part1_w$} {part2:>part2_w$} {name:<name_w$}"
        );
    }

    Ok(buffer)
}

pub fn render_day(session: &str, year: i32, day: u32, id: &str) -> Result<String> {
    if day < 1 || day > 25 {
        return Err(Error::response(Response::not_found()));
    }

    let leaderboard = get_leaderboard(session, year, id)?;

    let query = parse_query()?;
    let sort_method = query.get_value("s").unwrap_or("total");

    let table = render_table(&leaderboard, sort_method, year, day)?;

    let mut total_link = query.clone();
    let mut part1_link = query.clone();
    let mut part2_link = query.clone();

    total_link.replace("s", "total".into());
    part1_link.replace("s", "part1".into());
    part2_link.replace("s", "part2".into());

    let sort_options = match sort_method.as_ref() {
        "part1" => format!(
            "=> ?{total_link} Sort by total total
=> ?{part2_link} Sort by part 2"
        ),
        "part2" => format!(
            "=> ?{total_link} Sort by total total
=> ?{part1_link} Sort by part 1"
        ),
        _ => format!(
            "=> ?{part1_link} Sort by part 1
=> ?{part2_link} Sort by part 2"
        ),
    };

    let sort_name = match sort_method {
        "part1" => "part 1",
        "part2" => "part 2",
        _ => "Total time",
    };

    Ok(format!("
=> https://adventofcode.com/{year}/leaderboard/private/view/{id} View the leaderboard on adventofcode.com

> Sorting by {sort_name}

```leaderboard table
{table}
```

{sort_options}
"))
}
