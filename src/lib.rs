extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate time;

use std::io;
use std::cell::{RefCell};
use std::rc::Rc;
use std::time::Duration;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use hyper::client::{Client, Request, Response, DefaultTransport as HttpStream};
use hyper::header::Connection;
use hyper::{Decoder, Encoder, Next};
use hyper::Method::{Get};

use url::Url;

use serde_json::Value;
use serde_json::builder::{ArrayBuilder, ObjectBuilder};

use std::io::Error as IoError;
use hyper::Error as HttpError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum RestError {
    UrlParseError(UrlParseError),
    HttpRequestError(HttpError),
    HttpIoError(IoError)
}

pub struct RestResponse {
    pub code: u16,
    pub status: hyper::status::StatusCode,
    pub headers: hyper::header::Headers,
    pub body: String,
}

pub struct RestClient;

impl RestClient {


}