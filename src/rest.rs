use error::Error;
use http;
use serializer::ToJsonString;

use std::time::Duration;
use std::collections::HashMap;

pub struct EndpointBuilder {
    url: String,
    timeout: Duration,
    header: HashMap<String,String>
}

pub struct Endpoint {
    inner: http::Endpoint
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
    inner: http::Request
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
            inner: http::Endpoint::new(self.url.to_owned(), self.timeout, self.header.clone()),
        }
    }
}

impl Client {
    pub fn execute(endpoint: Endpoint, request: Request) -> Result<String, Error> {
        Ok(http::Client::request(endpoint.inner, request.inner))
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

    fn parse_route(&self) -> String {
        let mut route = self.path.to_owned();

        if self.urlsegment.len() >= 1 {
            for (key, val) in self.urlsegment.iter() {
                let format_key = String::new() + "{" + key + "}";
                if self.path.contains(&format_key) {
                    route = route.replace(&format_key, val);
                }
            }
        }

        if self.parameter.len() >= 1 {
            route.push_str("?");
            for(key, val) in self.parameter.iter() {
                let format_query = String::new() + key + "=" + val;
                route.push_str(&format_query);
            }
        }
        route
    }

    pub fn build(&self) -> Request {
        Request {
            inner: http::Request::new(http::Method::Get, Some(self.parse_route()), None)
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

    fn get_body(&self) -> Option<String> {
         Some(self.parameter.tojson())
    }

    pub fn build(&self) -> Request {
        Request {
            inner: http::Request::new(http::Method::Post, Some("posts".to_string()), self.get_body())
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
            inner: http::Request::new(http::Method::Put, None, None)
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
            inner: http::Request::new(http::Method::Patch, None, None)
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
            inner: http::Request::new(http::Method::Delete, None, None)
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
    let mut builder = Request::get();

    //Check we can set path
    builder.path("users/{id}");
    assert_eq!(builder.path, "users/{id}");

    //This should over write the existing path
    builder.path("posts/{id}");
    assert_eq!(builder.path, "posts/{id}");
}

#[test]
fn get_builder_url_segment() {
    let mut builder = Request::get();
    builder.path("users/{userId}/comments/{commentId}");

    //Check add url segment
    builder.add_urlsegment("userId", "1");

    assert_eq!(builder.urlsegment.len(), 1);
    assert_eq!(builder.urlsegment.contains_key("userId"), true);

    //Check add url segment
    builder.add_urlsegment("commentId", "7");

    assert_eq!(builder.urlsegment.len(), 2);
    assert_eq!(builder.urlsegment.contains_key("userId"), true);
    assert_eq!(builder.urlsegment.contains_key("commentId"), true);

    assert_eq!(&builder.parse_route(), "users/1/comments/7");
}

#[test]
fn get_builder_parameter() {
    let mut builder = Request::get();
    builder.path("posts");

    //Check add parameter
    builder.add_parameter("userId", "1");

    assert_eq!(builder.parameter.len(), 1);
    assert_eq!(builder.parameter.contains_key("userId"), true);

    assert_eq!(&builder.parse_route(), "posts?userId=1");
}

#[test]
fn get_builder_url_segment_paramter() {
    let mut builder = Request::get();
    builder.path("posts/{postId}");

    //Check add url segment & add parameter combo
    builder.add_urlsegment("postId", "1");
    builder.add_parameter("userId", "1");

    assert_eq!(builder.urlsegment.len(), 1);
    assert_eq!(builder.urlsegment.contains_key("postId"), true);

    assert_eq!(builder.parameter.len(), 1);
    assert_eq!(builder.parameter.contains_key("userId"), true);

    assert_eq!(&builder.parse_route(), "posts/1?userId=1");
}

#[test]
fn post_builder_parameter() {
    let mut builder = Request::post();
    builder.path("post");

    //Check add parameter
    builder.add_parameter("title", "foo");

    assert_eq!(builder.parameter.len(), 1);
    assert_eq!(builder.parameter.contains_key("title"), true);

    let x = builder.get_body().unwrap();
    println!("{}", x);
    //assert_eq!(&builder.get_body(), "posts?userId=1");
}
