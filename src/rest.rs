use error::Error;
use http;
use serializer;

use std::time::Duration;
use std::collections::BinaryHeap;
use std::collections::HashMap;

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

pub struct GetBuilder {
    path: String,
    urlsegment: HashMap<String,String>,
    parameter: HashMap<String,String>
}

pub struct PostBuilder {
    path: String,
    parameter: HashMap<String,String>
}

pub struct PutBuilder {
    path: String,
    parameter: HashMap<String,String>
}

pub struct PatchBuilder {
    path: String,
    urlsegment: HashMap<String,String>,
    parameter: HashMap<String,String>
}

pub struct DeleteBuilder {
    path: String,
    urlsegment: HashMap<String,String>
}

pub struct Request {
    method: http::Method,
    route: String,
    body: String
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

    fn validate() -> Result<String, Error> {
        Ok(String::new())
    }

    fn payload() -> String {
        String::new()
    }
}

impl GetBuilder {
    pub fn new() -> GetBuilder
    {
        GetBuilder {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }

    pub fn path(&mut self, path: &str) -> &mut GetBuilder {
        self.path.clear();
        self.path.push_str(path);
        self
    }

    pub fn add_urlsegment(&mut self, urlsegment: &str, value: &str) -> &mut GetBuilder {
        self.urlsegment.entry(urlsegment.to_string()).or_insert(value.to_string());
        self
    }

    pub fn add_parameter(&mut self, parameter: &str, value: &str) -> &mut GetBuilder {
        self.parameter.entry(parameter.to_string()).or_insert(value.to_string());
        self
    }

    pub fn build(&self) -> Request {
        Request {
            method: http::Method::Get,
            route: String::new(),
            body: String::new()
        }
    }
}

impl PostBuilder {
    pub fn new() -> PostBuilder
    {
        PostBuilder {
            path: String::new(),
            parameter: HashMap::new()
        }
    }

    pub fn path(&mut self, path: &str) -> &mut PostBuilder {
        self.path.push_str(path);
        self
    }

    pub fn add_parameter(&mut self, parameter: &str, value: &str) -> &mut PostBuilder {
        self.parameter.entry(parameter.to_string()).or_insert(value.to_string());
        self
    }

    pub fn build(&self) -> Request {
        Request {
            method: http::Method::Post,
            route: String::new(),
            body: String::new()
        }
    }
}

impl PutBuilder {
    pub fn new() -> PutBuilder
    {
        PutBuilder {
            path: String::new(),
            parameter: HashMap::new()
        }
    }

    pub fn path(&mut self, path: &str) -> &mut PutBuilder {
        self.path.push_str(path);
        self
    }

    pub fn add_parameter(&mut self, parameter: &str, value: &str) -> &mut PutBuilder {
        self.parameter.entry(parameter.to_string()).or_insert(value.to_string());
        self
    }

    pub fn build(&self) -> Request {
        Request {
            method: http::Method::Put,
            route: String::new(),
            body: String::new()
        }
    }
}

impl PatchBuilder {
    pub fn new() -> PatchBuilder
    {
        PatchBuilder {
            path: String::new(),
            urlsegment: HashMap::new(),
            parameter: HashMap::new()
        }
    }

    pub fn path(&mut self, path: &str) -> &mut PatchBuilder {
        self.path.push_str(path);
        self
    }

    pub fn add_urlsegment(&mut self, urlsegment: &str, value: &str) -> &mut PatchBuilder {
        self.urlsegment.entry(urlsegment.to_string()).or_insert(value.to_string());
        self
    }

    pub fn add_parameter(&mut self, parameter: &str, value: &str) -> &mut PatchBuilder {
        self.parameter.entry(parameter.to_string()).or_insert(value.to_string());
        self
    }

    pub fn build(&self) -> Request {
        Request {
            method: http::Method::Patch,
            route: String::new(),
            body: String::new()
        }
    }
}

impl DeleteBuilder {
    pub fn new() -> DeleteBuilder
    {
        DeleteBuilder {
            path: String::new(),
            urlsegment: HashMap::new()
        }
    }

    pub fn path(&mut self, path: &str) -> &mut DeleteBuilder {
        self.path.push_str(path);
        self
    }

    pub fn add_urlsegment(&mut self, urlsegment: &str, value: &str) -> &mut DeleteBuilder {
        self.urlsegment.entry(urlsegment.to_string()).or_insert(value.to_string());
        self
    }

    pub fn build(&self) -> Request {
        Request {
            method: http::Method::Delete,
            route: String::new(),
            body: String::new()
        }
    }
}

impl Request {
    pub fn get() -> GetBuilder {
        GetBuilder::new()
    }

    pub fn post() -> PostBuilder {
        PostBuilder::new()
    }

    pub fn put() -> PutBuilder {
        PutBuilder::new()
    }

    pub fn patch() -> PatchBuilder {
        PatchBuilder::new()
    }

    pub fn delete() -> DeleteBuilder {
        DeleteBuilder::new()
    }
}


#[test]
fn get_builder_path() {
    let mut builder = GetBuilder::new();

    //Check we can set path
    builder.path("users/{id}");
    assert_eq!(builder.path, "users/{id}");

    //This should over write the existing path
    builder.path("posts/{id}");
    assert_eq!(builder.path, "posts/{id}");

}