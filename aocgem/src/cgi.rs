use std::{env, error::Error, fmt::Display, result};
use url::{self, Url};
use urlencoding::decode;
use std::string::FromUtf8Error;

pub type Result = result::Result<Response, Box<dyn Error>>;

pub enum Response {
    Input(String),
    Success(String, String),
    Redirect(String),
    CgiError(String),
    TempError(String),
    PermError(String),
    NotFound,
}

impl Response {
    #[inline]
    fn first_line(meta: impl AsRef<str>, default: impl AsRef<str>) -> String {
        meta.as_ref()
            .lines()
            .next()
            .unwrap_or(default.as_ref())
            .to_owned()
    }

    pub fn input(prompt: impl AsRef<str>) -> Self {
        Self::Input(Self::first_line(prompt, ""))
    }

    pub fn success(mime: impl AsRef<str>, body: impl Into<String>) -> Self {
        Self::Success(Self::first_line(mime, "text/gemini"), body.into())
    }

    pub fn redirect(dest: impl AsRef<str>) -> Self {
        Self::Redirect(Self::first_line(dest, ""))
    }

    pub fn cgi_error(message: impl Display) -> Self {
        Self::CgiError(Self::first_line(
            message.to_string(),
            "Internal Server Error",
        ))
    }

    pub fn temp_error(message: impl Display) -> Self {
        Self::TempError(Self::first_line(message.to_string(), "Temporary Error"))
    }

    pub fn perm_error(message: impl Display) -> Self {
        Self::PermError(Self::first_line(message.to_string(), "Permanent Error"))
    }

    #[inline]
    pub fn not_found() -> Self {
        Self::NotFound
    }

    pub fn status_code(&self) -> i32 {
        match self {
            Response::Input(_) => 10,
            Response::Success(_, _) => 20,
            Response::Redirect(_) => 30,
            Response::CgiError(_) => 42,
            Response::TempError(_) => 40,
            Response::PermError(_) => 50,
            Response::NotFound => 51,
        }
    }

    pub fn meta(&self) -> &str {
        match self {
            Response::Input(prompt) => prompt,
            Response::Success(mime, _) => mime,
            Response::Redirect(dest) => dest,
            Response::CgiError(message) => message,
            Response::TempError(message) => message,
            Response::PermError(message) => message,
            Response::NotFound => "Not Found",
        }
    }

    pub fn body(&self) -> &str {
        match self {
            Response::Success(_, body) => body,
            _ => "",
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = self.status_code();
        let meta = self.meta();
        let body = self.body();
        f.write_fmt(format_args!("{status} {meta}\r\n{body}"))
    }
}

#[inline]
pub fn get_url() -> result::Result<Url, Box<dyn Error>> {
    let gemini_url = env::var("GEMINI_URL").unwrap();
    Ok(Url::parse(&gemini_url)?)
}

#[inline]
pub fn get_query() -> result::Result<String, FromUtf8Error> {
    Ok(decode(&env::var("QUERY_STRING").unwrap())?.into_owned())
}

#[inline]
pub fn get_path() -> String {
    env::var("PATH_INFO").unwrap()
}

#[inline]
pub fn get_script() -> String {
    env::var("SCRIPT_NAME").unwrap()
}
