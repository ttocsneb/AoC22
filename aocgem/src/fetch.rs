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

use crate::leaderboard::{Leaderboard, PublicLeaderboard};

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
                "Invalid leaderboard id",
            ))),
        }
    } else if response.status().is_client_error() {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid year or leaderboard id",
        )))
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid Session",
        )))
    }
}

#[inline]
fn get_cache_path() -> PathBuf {
    PathBuf::from(option_env!("DATA_DIR").unwrap_or("data"))
}

fn get_data_path(group: &str, year: i32) -> PathBuf {
    let mut path = get_cache_path();
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

fn get_pub_data_path(id: &str) -> PathBuf {
    let mut path = get_cache_path();
    path.push("pub");
    path.push(format!("{id}.json"));
    path
}

pub fn load_pub_leaderboard(id: &str) -> Result<PublicLeaderboard, Box<dyn Error>> {
    let path = get_pub_data_path(id);

    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}

pub fn save_pub_leaderboard(
    id: &str,
    leaderboard: &PublicLeaderboard,
) -> Result<(), Box<dyn Error>> {
    let path = get_pub_data_path(id);
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    let val = serde_json::to_string(&leaderboard)?;
    f.write_all(val.as_bytes())?;
    Ok(())
}

#[inline]
pub fn pub_leaderboard_exists(id: &str) -> bool {
    get_pub_data_path(id).exists()
}

pub fn find_pub_leaderboard(group: &str) -> Result<Option<PublicLeaderboard>, Box<dyn Error>> {
    let mut path = get_cache_path();
    path.push("pub");
    if !path.exists() {
        return Ok(None);
    }

    for child in path.read_dir()? {
        let child = child?.path();
        if !child.is_file() {
            continue;
        }
        let mut f = File::open(&child)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        if let Ok(pub_board) = serde_json::from_str::<PublicLeaderboard>(&contents) {
            if pub_board.id == group {
                return Ok(Some(pub_board));
            }
        }
    }

    Ok(None)
}
