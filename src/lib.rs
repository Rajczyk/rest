extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate time;

use std::io;
use std::sync::RwLock;
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
    code: u16,
    status: hyper::status::StatusCode,
    headers: hyper::header::Headers,
    pub body: String,
}

pub enum Method {
    Get,
    Post,
    Put
}

pub struct RestRequest {

}

pub struct Endpoint {
    url: String,
}

pub struct RestClient {
    endpoint: Endpoint,
    query: String
}

impl Endpoint {
    pub fn configure() -> Endpoint {
        Endpoint {  url: String::new() }
    }

    pub fn url(&mut self, url: &str) -> &mut Endpoint {
        self.url.push_str(url);
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Endpoint {
        self
    }

    pub fn build(&self) -> RestClient {
        RestClient::new(self)
    }
}

impl Clone for Endpoint {
    fn clone(&self) -> Self {
        Endpoint{ url: self.url.to_owned() }
    }
}

impl RestClient {
    fn new(endpoint: &Endpoint) -> RestClient {
        RestClient { endpoint: endpoint.to_owned(), query: String::new() }
    }

    pub fn request(self, method: Method) -> RestRequest {
        RestRequest::new(method)
    }

    pub fn query(self, query: &str) -> RestClient {
        self.query.push_str(query);
        self
    }

    pub fn get(&self) -> Result<RestResponse, RestError> {
        Ok(RestResponse {
            code: 1,
            status: hyper::status::StatusCode::Accepted,
            headers: hyper::header::Headers::default(),
            body: "".to_string()
        })
    }
}

impl RestRequest {
    fn new(method: Method) -> RestRequest
    {

    }

}