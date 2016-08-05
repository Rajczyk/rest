use error::Error;
use http;
use serializer;

use std::time::Duration;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct RequestBuilder {
    path: String,
    urlsegment: HashMap<String,String>,
    parameter: HashMap<String,String>
}

pub struct Request {
    path: String,
    urlsegment: HashMap<String,String>,
    parameter: HashMap<String,String>
}

pub struct EndpointBuilder {
    url: String,
    timeout: Duration,
    header: HashMap<String,String>
}

pub struct Endpoint {
    inner: http::Endpoint,
    url: String,
    header: HashMap<String,String>,
}

pub struct Client {

}

impl Endpoint {
    pub fn configure() -> EndpointBuilder {
        EndpointBuilder {
            url: String::new(),
            timeout: Duration::from_secs(10),
            header: HashMap::new()}
    }
}

impl EndpointBuilder
{
    pub fn url(&mut self, url: &str) -> &mut EndpointBuilder {
        self.url.push_str(url);
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut EndpointBuilder {
        self.timeout = timeout;
        self
    }

    pub fn add_header(&mut self, header: &str, value: &str) -> &mut EndpointBuilder {
        self.header.entry(header.to_string()).or_insert(value.to_string());
        self
    }

    pub fn build(&self) -> Endpoint {
        Endpoint {
            inner: http::Endpoint::new(self.timeout),
            url: self.url.to_owned(),
            header: self.header.clone(),
        }
    }
}

impl Client {
    pub fn execute(endpoint: Endpoint, request: Request) -> Result<String, Error> {
        Ok(http::Client::request(endpoint.inner, &endpoint.url))
    }
}

impl RequestBuilder {
    pub fn path(&mut self, path: &str) -> &mut RequestBuilder {
        self.path.push_str(path);
        self
    }

    pub fn add_urlsegment(&mut self, urlsegment: &str, value: &str) -> &mut RequestBuilder {
        self
    }

    pub fn add_parameter(&mut self, parameter: &str, value: &str) -> &mut RequestBuilder {
        self
    }

    pub fn build(&self) -> Request {
        Request::new()
    }
}

impl Request {
    fn new() -> Request {
        Request {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }

    pub fn get() -> RequestBuilder {
        RequestBuilder {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }

    pub fn post() -> RequestBuilder {
        RequestBuilder {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }

    pub fn put() -> RequestBuilder {
        RequestBuilder {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }

    pub fn patch() -> RequestBuilder {
        RequestBuilder {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }

    pub fn delete() -> RequestBuilder {
        RequestBuilder {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }
}