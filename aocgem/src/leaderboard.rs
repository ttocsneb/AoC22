use std::collections::HashMap;

use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

pub fn est_offset() -> FixedOffset {
    FixedOffset::west_opt(5 * 3600).unwrap()
}

pub fn est_midnight(day: NaiveDate) -> DateTime<FixedOffset> {
    let midnight = NaiveDateTime::new(
        day,
        NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap(),
    );
    DateTime::from_local(midnight, est_offset())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Completion {
    pub get_star_ts: i64,
    pub star_index: i32,
}

impl Completion {
    pub fn completion_time(&self) -> DateTime<FixedOffset> {
        let dt = NaiveDateTime::from_timestamp_opt(self.get_star_ts, 0).unwrap();
        DateTime::from_utc(dt, est_offset())
    }

    pub fn duration(&self, start_date: NaiveDate) -> Duration {
        let time = self.completion_time();
        let midnight = est_midnight(start_date);

        time - midnight
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub global_score: i32,
    pub last_star_ts: i64,
    pub local_score: i32,
    pub stars: i32,
    pub completion_day_level: HashMap<String, HashMap<String, Completion>>,
}

impl Member {
    fn calc_completion_time(
        year: i32,
        day: u32,
        completion: &HashMap<String, Completion>,
    ) -> (Option<Duration>, Option<Duration>) {
        let start_date = match NaiveDate::from_ymd_opt(year, 12, day) {
            Some(val) => val,
            None => {
                return (None, None);
            }
        };
        if let Some(a) = completion.get("1") {
            let x = a.duration(start_date);
            let y = completion
                .get("2")
                .map(|b| b.completion_time() - a.completion_time());
            (Some(x), y)
        } else {
            (None, None)
        }
    }

    pub fn completion_time(&self, day: u32, year: i32) -> (Option<Duration>, Option<Duration>) {
        if let Some(completion) = self.completion_day_level.get(&day.to_string()) {
            Self::calc_completion_time(year, day, completion)
        } else {
            (None, None)
        }
    }

    pub fn completion_times(
        &self,
        year: i32,
    ) -> HashMap<u32, (Option<Duration>, Option<Duration>)> {
        let mut times = HashMap::new();
        for (day, completion) in &self.completion_day_level {
            if let Ok(day) = u32::from_str_radix(day, 10) {
                times.insert(day, Self::calc_completion_time(year, day, completion));
            }
        }
        times
    }

    pub fn total_completion_time(&self, year: i32) -> Duration {
        let mut total = Duration::seconds(0);
        for (day, completion) in &self.completion_day_level {
            if let Ok(day) = u32::from_str_radix(day, 10) {
                let (a, b) = Self::calc_completion_time(year, day, completion);
                if let Some(a) = a {
                    total = total.checked_add(&a).unwrap();
                }
                if let Some(b) = b {
                    total = total.checked_add(&b).unwrap();
                }
            }
        }
        total
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Leaderboard {
    pub event: String,
    pub owner_id: i32,
    pub members: HashMap<String, Member>,
}
