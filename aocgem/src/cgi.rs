use lazy_static::lazy_static;
use std::{env, error, fmt::Display, result};
use url::{self, Url};

use crate::query::Query;

pub type Result<T> = result::Result<T, Error>;

pub trait OkResponse<T> {
    fn ok_or_response(self, response: Response) -> result::Result<T, Error>;
    fn ok_else_response<F: FnOnce() -> Response>(self, response: F) -> result::Result<T, Error>;
}

impl<T, E> OkResponse<T> for result::Result<T, E>
where
    E: Display,
{
    fn ok_or_response(self, response: Response) -> result::Result<T, Error> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::response_with_message(e.to_string(), response)),
        }
    }

    fn ok_else_response<F: FnOnce() -> Response>(self, response: F) -> result::Result<T, Error> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::response_with_message(e.to_string(), response())),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Message(String),
    Response(String, Response),
    Nested(Box<dyn error::Error>),
    NestedResponse(Box<dyn error::Error>, Response),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Nested(err) => Some(err.as_ref()),
            Error::NestedResponse(err, _) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl Error {
    #[inline]
    pub fn nested(err: Box<dyn error::Error>) -> Self {
        Self::Nested(err)
    }

    pub fn nested_with_response(err: Box<dyn error::Error>, response: Response) -> Self {
        Self::NestedResponse(err, response)
    }

    #[inline]
    pub fn from_err<E: error::Error + 'static>(err: E) -> Self {
        Self::Nested(Box::new(err))
    }

    #[inline]
    pub fn from_err_with_response(err: impl error::Error + 'static, response: Response) -> Self {
        Self::NestedResponse(Box::new(err), response)
    }

    #[inline]
    pub fn response(response: Response) -> Self {
        Self::Response(response.meta().into(), response)
    }

    #[inline]
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }

    #[inline]
    pub fn response_with_message(message: impl Into<String>, response: Response) -> Self {
        Self::Response(message.into(), response)
    }

    pub fn get_response(&self) -> Option<&Response> {
        match self {
            Error::Message(_) => None,
            Error::Nested(_) => None,
            Error::NestedResponse(_, response) => Some(response),
            Error::Response(_, response) => Some(response),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::Response(msg, _) => f.write_str(msg),
            Error::Nested(err) => err.fmt(f),
            Error::NestedResponse(err, _) => err.fmt(f),
        }
    }
}

impl From<Error> for Response {
    fn from(err: Error) -> Self {
        match err {
            Error::Response(_, response) => response,
            Error::NestedResponse(_, response) => response,
            _ => Response::cgi_error("Internal Server Error"),
        }
    }
}

impl From<Box<dyn error::Error>> for Error {
    fn from(err: Box<dyn error::Error>) -> Self {
        Self::nested(err)
    }
}

#[derive(Debug)]
pub enum ResponseCode {
    Input,
    SensitiveInput,
    Success,
    Redirect,
    RedirectPerm,
    TempError,
    Unavailable,
    CgiError,
    ProxyError,
    SlowDown,
    PermError,
    NotFound,
    Gone,
    ProxyRefused,
    BadRequest,
    CertRequired,
    CertNotAuthorized,
    CertNotValid,
}

impl ResponseCode {
    fn status_code(&self) -> i32 {
        match self {
            ResponseCode::Input => 10,
            ResponseCode::SensitiveInput => 11,
            ResponseCode::Success => 20,
            ResponseCode::Redirect => 30,
            ResponseCode::RedirectPerm => 31,
            ResponseCode::TempError => 40,
            ResponseCode::Unavailable => 41,
            ResponseCode::CgiError => 42,
            ResponseCode::ProxyError => 43,
            ResponseCode::SlowDown => 44,
            ResponseCode::PermError => 50,
            ResponseCode::NotFound => 51,
            ResponseCode::Gone => 52,
            ResponseCode::ProxyRefused => 53,
            ResponseCode::BadRequest => 59,
            ResponseCode::CertRequired => 60,
            ResponseCode::CertNotAuthorized => 61,
            ResponseCode::CertNotValid => 62,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    code: ResponseCode,
    meta: String,
    body: Option<String>,
}

impl Response {
    fn new(code: ResponseCode, meta: impl Into<String>) -> Self {
        Self {
            code,
            meta: meta.into(),
            body: None,
        }
    }

    #[inline]
    pub fn input(prompt: impl Into<String>) -> Self {
        Self::new(ResponseCode::Input, prompt)
    }

    #[inline]
    pub fn sensitive_input(prompt: impl Into<String>) -> Self {
        Self::new(ResponseCode::SensitiveInput, prompt)
    }

    pub fn success(mime: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            code: ResponseCode::Success,
            meta: mime.into(),
            body: Some(body.into()),
        }
    }

    #[inline]
    pub fn redirect(dest: impl Into<String>) -> Self {
        Self::new(ResponseCode::Redirect, dest)
    }

    #[inline]
    pub fn redirect_perm(dest: impl Into<String>) -> Self {
        Self::new(ResponseCode::RedirectPerm, dest)
    }

    #[inline]
    pub fn temp_error(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::TempError, message)
    }

    #[inline]
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::Unavailable, message)
    }

    #[inline]
    pub fn cgi_error(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::CgiError, message)
    }

    #[inline]
    pub fn proxy_error(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::ProxyError, message)
    }

    #[inline]
    pub fn slow_down(delay: i32) -> Self {
        Self::new(ResponseCode::SlowDown, delay.to_string())
    }

    #[inline]
    pub fn perm_error(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::PermError, message)
    }

    #[inline]
    pub fn not_found() -> Self {
        Self::new(ResponseCode::NotFound, "Path not found")
    }

    #[inline]
    pub fn gone() -> Self {
        Self::new(ResponseCode::Gone, "Path no longer exists")
    }

    #[inline]
    pub fn proxy_refused(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::ProxyRefused, message)
    }

    #[inline]
    pub fn bad_request() -> Self {
        Self::new(ResponseCode::BadRequest, "Invalid request")
    }

    #[inline]
    pub fn cert_required(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::CertRequired, message)
    }

    #[inline]
    pub fn cert_not_authorized(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::CertNotAuthorized, message)
    }

    #[inline]
    pub fn cert_not_valid(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::CertNotValid, message)
    }

    #[inline]
    pub fn status_code(&self) -> i32 {
        self.code.status_code()
    }

    pub fn meta(&self) -> &str {
        &self.meta
    }

    pub fn body(&self) -> &str {
        if let Some(body) = &self.body {
            body
        } else {
            ""
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

pub fn get_url() -> &'static str {
    lazy_static! {
        static ref GEMINI_URL: String = env::var("GEMINI_URL").unwrap();
    }
    &GEMINI_URL
}

pub fn parse_url() -> Result<Url> {
    Url::parse(get_url()).ok_else_response(|| Response::bad_request())
}

pub fn get_query() -> &'static str {
    lazy_static! {
        static ref QUERY_STRING: String = env::var("QUERY_STRING").unwrap();
    }

    &QUERY_STRING
}

pub fn parse_query() -> Result<Query> {
    Query::parse(get_query()).ok_else_response(|| Response::bad_request())
}

pub fn get_path() -> &'static str {
    lazy_static! {
        static ref PATH_INFO: String = env::var("PATH_INFO").unwrap();
    }

    &PATH_INFO
}

pub fn get_script() -> &'static str {
    lazy_static! {
        static ref SCRIPT_NAME: String = env::var("SCRIPT_NAME").unwrap();
    }

    &SCRIPT_NAME
}
