use std::{cmp::Ordering, error::Error, time::SystemTime};

use crate::{
    cgi::{get_path, get_script, parse_query},
    fetch::get_leaderboard,
    leaderboard::{est_offset, Leaderboard, Member},
};

use super::render_duration;

use ansi_term::{Color, Style};
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, Utc};

fn render_days(member: &Member, year: i32, render_color: bool) -> String {
    let times = member.completion_times(year);

    let mut foobar = [0; 25];

    for day in 0..25 {
        if let Some((a, b)) = times.get(&(day + 1)) {
            if let Some(_) = a {
                foobar[day as usize] += 1;
            }
            if let Some(_) = b {
                foobar[day as usize] += 1;
            }
        }
    }

    let mut buffer = String::new();
    let mut color = Style::default();

    for day in 0..25 {
        if render_color {
            let next_color = match foobar[day] {
                1 => Style::default(),
                2 => Color::Yellow.normal(),
                _ => Color::Fixed(8).blink(),
            };
            buffer += &color.infix(next_color).to_string();
            color = next_color;
        }
        buffer += match foobar[day] {
            1 => "+",
            2 => "*",
            _ => "-",
        };
    }
    if render_color {
        buffer += &color.infix(Style::default()).to_string();
    }

    buffer
}

fn render_members(
    leaderboard: &Leaderboard,
    sort_method: &str,
    render_color: bool,
) -> Result<String, Box<dyn Error>> {
    let year = i32::from_str_radix(&leaderboard.event, 10)?;
    let mut elements = Vec::new();

    for (_, member) in &leaderboard.members {
        let total_time = member.total_completion_time(year);
        let average_time = match member.stars == 0 {
            true => total_time,
            false => match total_time {
                Some(t) => Some(t / member.stars),
                None => None,
            },
        };
        elements.push((
            // Sorting elements
            member.local_score,
            average_time.map(|t| t.num_seconds()),
            member.global_score,
            member.stars,
            // Data
            member.local_score.to_string(),
            member.global_score.to_string(),
            member.stars.to_string(),
            render_days(member, year, render_color),
            match total_time {
                Some(d) => render_duration(&d),
                None => "--:--:--".to_owned(),
            },
            match average_time {
                Some(d) => render_duration(&d),
                None => "--:--:--".to_owned(),
            },
            &member.name,
        ));
    }

    match sort_method {
        "stars" => elements.sort_unstable_by(|a, b| match a.3.cmp(&b.3) {
            Ordering::Equal => match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
            },
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
        }),
        "global" => elements.sort_unstable_by(|a, b| match a.2.cmp(&b.2) {
            Ordering::Equal => match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
            },
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
        }),
        "time" => elements.sort_unstable_by(|a, b| a.1.cmp(&b.1)),
        _ => elements.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
        }),
    };

    let mut local_w = 0;
    let mut global_w = 0;
    let mut stars_w = 0;
    let mut dur_w = 0;
    let mut avg_w = 0;
    let mut name_w = 0;
    for (_, _, _, _, local, global, stars, _days, dur, avg, name) in &elements {
        local_w = local.len().max(local_w);
        global_w = global.len().max(global_w);
        stars_w = stars.len().max(stars_w);
        dur_w = dur.len().max(dur_w);
        avg_w = avg.len().max(avg_w);
        name_w = name.len().max(name_w);
    }

    let mut buffer = String::new();

    let n_t = "";
    let score_t = "Score";
    let stars_t = "Stars";
    let days_t = "1234567890123456789012345";
    let dur_t = "Total Time";
    let avg_t = "Average";
    let name_t = "Name";

    local_w = local_w.max(score_t.len() / 2);
    global_w = global_w.max(score_t.len() / 2);
    stars_w = stars_w.max(stars_t.len());
    dur_w = dur_w.max(dur_t.len());
    avg_w = avg_w.max(avg_t.len());
    name_w = name_w.max(name_t.len());

    let n_w = elements.len().to_string().len() + 1;
    let score_w = local_w + global_w + 1;
    let pre_w = n_w + score_w + stars_w + 2;
    buffer += &format!("{n_t:<pre_w$}          1111111111222222");
    buffer += &format!(
        "\n{n_t:<n_w$} {score_t:^score_w$} {stars_t:<stars_w$} {days_t} {dur_t:<dur_w$} {avg_t:<avg_w$} {name_t:<name_w$}"
    );

    for (i, (_, _, _, _, local, global, stars, days, dur, avg, name)) in
        elements.into_iter().enumerate()
    {
        let i = i + 1;
        let i = format!("{i}.");
        buffer += &format!("\n{i:>n_w$} {local:>local_w$}:{global:<global_w$} {stars:<stars_w$} {days} {dur:>dur_w$} {avg:>avg_w$} {name:<name_w$}");
    }

    Ok(buffer)
}

pub fn render_leaderboard(session: &str, year: i32, id: &str) -> Result<String, Box<dyn Error>> {
    let leaderboard = get_leaderboard(session, year, id)?;

    let query = parse_query()?;
    let sort_method = query.get_value("s").unwrap_or("local");
    let render_color = query.contains("c");

    let scores = render_members(&leaderboard, &sort_method, render_color)?;

    let mut global_link = query.clone();
    let mut local_link = query.clone();
    let mut stars_link = query.clone();
    let mut time_link = query.clone();

    global_link.replace("s", "global".into());
    local_link.replace("s", "local".into());
    stars_link.replace("s", "stars".into());
    time_link.replace("s", "time".into());

    let sort_name = match sort_method {
        "stars" => "stars",
        "global" => "global score",
        "time" => "time",
        _ => "local score",
    };

    let sort_options = match sort_method.as_ref() {
        "stars" => format!(
            "=> ?{global_link} Sort by global score
=> ?{local_link} Sort by local score
=> ?{time_link} Sort by time",
        ),
        "global" => format!(
            "=> ?{stars_link} Sort by stars
=> ?{local_link} Sort by local score
=> ?{time_link} Sort by time"
        ),
        "time" => format!(
            "=> ?{global_link} Sort by global score
=> ?{local_link} Sort by local score
=> ?{stars_link} Sort by stars"
        ),
        _ => format!(
            "=> ?{global_link} Sort by global score
=> ?{stars_link} Sort by stars
=> ?{time_link} Sort by time"
        ),
    };

    let mut color_select = query.clone();
    let color_name = if render_color {
        color_select.erase("c");
        if color_select.is_empty() {
            color_select.replace("s", "time".into());
        }
        "Disable Colors"
    } else {
        color_select.push("c".into());
        "Enable Colors"
    };

    let now = DateTime::<Utc>::from(SystemTime::now());
    let now = DateTime::<FixedOffset>::from_utc(now.naive_utc(), est_offset());
    let today = now.date_naive();

    let latest = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();
    let latest_day = match today > latest {
        true => latest.day(),
        false => today.day(),
    };

    let script = get_script();
    let path = get_path();
    Ok(format!("
=> https://adventofcode.com/{year}/leaderboard/private/view/{id} View the leaderboard on adventofcode.com

> Sorting by {sort_name}

```leaderboard table
{scores}
```

=> ?{color_select} {color_name}

{sort_options}

## View times for specific days

You can view statistics for a specific day's problem.

=> {script}{path}/{latest_day}/ View day {latest_day}'s stats
=> {script}{path}/day/ Select a day to view

"))
}
