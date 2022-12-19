use chrono::Duration;

pub mod day;
pub mod main;

fn render_duration(duration: &Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours() % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
