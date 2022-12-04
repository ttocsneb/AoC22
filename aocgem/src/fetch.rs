use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    time::{Duration, SystemTime},
};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};

use crate::leaderboard::Leaderboard;

pub fn fetch_leaderboard(
    session: &str,
    group: &str,
    year: i32,
) -> Result<Leaderboard, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{year}/leaderboard/private/view/{group}.json");

    let mut headers = HeaderMap::new();
    headers.insert(
        "cookie",
        HeaderValue::from_str(&format!("session={session}"))?,
    );
    let client = Client::builder().default_headers(headers).build()?;
    let request = client.get(url).build()?;
    let response = client.execute(request)?;

    if response.status().is_success() {
        match response.json() {
            Ok(val) => Ok(val),
            Err(_) => Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid group id",
            ))),
        }
    } else if response.status().is_client_error() {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid year",
        )))
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid Session",
        )))
    }
}

fn get_data_path(group: &str, year: i32) -> PathBuf {
    let mut path = PathBuf::from(env!("CACHE_DIR"));
    path.push(format!("{group}-{year}.json"));
    return path;
}

pub fn save_leaderboard(
    leaderboard: &Leaderboard,
    group: &str,
    year: i32,
) -> Result<(), Box<dyn Error>> {
    let path = get_data_path(group, year);
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
    }
    let mut f = File::create(path)?;
    let val = serde_json::to_string(&leaderboard)?;
    f.write_all(val.as_bytes())?;
    Ok(())
}

pub fn load_leaderboard(group: &str, year: i32) -> Result<Leaderboard, Box<dyn Error>> {
    let path = get_data_path(group, year);
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}

pub fn get_age(group: &str, year: i32) -> Result<Duration, Box<dyn Error>> {
    let path = get_data_path(group, year);
    if !path.exists() {
        return Ok(Duration::MAX);
    }
    let time = path.metadata()?.modified()?;
    Ok(SystemTime::now().duration_since(time)?)
}
