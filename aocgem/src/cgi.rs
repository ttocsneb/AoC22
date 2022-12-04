use std::{
    env,
    error::Error,
    fmt::Display,
    io::{self, Write},
};
use url::{self, Url};

pub fn success_stream<R>(mime: &str, mut content: R) -> io::Result<()>
where
    R: io::Read,
{
    print!("20 {mime}\r\n");
    let mut buf = [0u8; 1400];
    loop {
        let read = content.read(&mut buf)?;
        if read == 0 {
            return Ok(());
        }
        io::stdout().write_all(&buf[..read])?;
    }
}

pub fn success(mime: &str, content: &str) {
    print!("20 {mime}\r\n{content}");
}

pub fn response(code: u32, err: impl Display) {
    if let Some(msg) = err.to_string().lines().next() {
        print!("{code} {msg}\r\n");
    } else {
        print!("{code}\r\n")
    }
}

#[inline]
pub fn get_url() -> Result<Url, Box<dyn Error>> {
    let gemini_url = env::var("GEMINI_URL")?;
    Ok(Url::parse(&gemini_url)?)
}

#[inline]
pub fn get_query() -> Result<String, env::VarError> {
    env::var("QUERY_STRING")
}

#[inline]
pub fn get_path() -> Result<String, env::VarError> {
    env::var("PATH_INFO")
}

#[inline]
pub fn get_script() -> Result<String, env::VarError> {
    env::var("SCRIPT_NAME")
}
