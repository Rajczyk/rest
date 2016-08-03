use http;
use serializer;

use std::time::Duration;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use std::io::Error as IoError;
use hyper::Error as HttpError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum Error {
    UrlParseError(UrlParseError),
    HttpRequestError(HttpError),
    HttpIoError(IoError)
}

//pub struct Response {
//    code: u16,
//   status: StatusCode,
//   headers: Headers,
//   pub body: String,
// }

pub enum Method {
    Get,
    Post,
    Put
}

pub struct RequestBuilder {
    query: String
}

pub struct Request {

}

pub struct EndpointBuilder {
    url: String,
}

pub struct Endpoint {
    endpoint: EndpointBuilder,
}

pub struct Client {

}

impl Endpoint {
    pub fn configure() -> EndpointBuilder {
        EndpointBuilder{ url: String::new() }
    }

    fn new(builder: &EndpointBuilder) -> Endpoint
    {
        Endpoint { endpoint: builder.to_owned()}
    }
}

impl EndpointBuilder
{
    pub fn url(&mut self, url: &str) -> &mut EndpointBuilder {
        self.url.push_str(url);
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut EndpointBuilder {
        self
    }

    pub fn build(&self) -> Endpoint {
        Endpoint::new(self)
    }
}

impl Clone for EndpointBuilder {
    fn clone(&self) -> Self {
        EndpointBuilder{ url: self.url.to_owned() }
    }
}

impl Client {
    pub fn execute(endpoint: Endpoint, request: Request) -> Result<String, Error> {

        let http_client = http::Client::new();;


        http_client.request("http://jsonplaceholder.typicode.com/posts/1");

        Ok(String::new())
        //Ok(Response {
        //   code: 1,
        //    status: StatusCode::Accepted,
        //    headers: Headers::default(),
        //    body: "".to_string()
        //})
    }
}

impl RequestBuilder {
    pub fn query(&mut self, query: &str) -> &mut RequestBuilder {
        self.query.push_str(query);
        self
    }

    pub fn build(&self) -> Request {
        Request::new()
    }
}

impl Request {
    fn new() -> Request {
        Request {}
    }

    pub fn get() -> RequestBuilder {
        RequestBuilder { query: String::new() }
    }
}