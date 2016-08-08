use std::io;
use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;

use hyper::client::{Request as HyperRequest, Response as HyperResponse, DefaultTransport as HttpStream};
use hyper::header::{Connection, Headers, UserAgent};
use hyper::{Decoder, Encoder, Next};
use hyper::status::StatusCode;
use hyper;

use url::Url;

pub type ResultSender = mpsc::Sender<(Request, Option<Response>)>;

#[derive(Debug, Clone)]
pub enum Method
{
    Get,
    Post,
    Patch,
    Put,
    Delete
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Option<Vec<u8>>
}

pub struct Handler {
    request: Request,
    response: Option<Response>,
    sender: ResultSender,
    user_agent: String,
}

#[derive(Debug, Clone)]
pub struct Request {
     method: hyper::Method,
     route: Option<String>,
     body: Option<String>
}

impl Method {
    fn to_hyper(&self) -> hyper::Method {
        match *self {
            Method::Get  => hyper::Method::Get,
            Method::Post => hyper::Method::Post,
            Method::Patch => hyper::Method::Patch,
            Method::Put => hyper::Method::Put,
            Method::Delete => hyper::Method::Delete
        }
    }
}

impl Request {
    pub fn new(method: Method, route: Option<String>, body: Option<String>) -> Self {
        Request {
            method: method.to_hyper(),
            route: route,
            body: body
        }
    }
}

impl Handler {
    fn read(&self) -> Next {
        Next::read().timeout(Duration::from_secs(1000))
    }

    fn return_response(&self) -> Next {
        self.send_result();
        Next::end()
    }

    fn send_result(&self) {
        self.sender.send((self.request.clone(), self.response.clone())).unwrap();
    }
}

pub struct Client {  }

pub struct Endpoint
{
    url: Url,
    client: hyper::Client<Handler>,
    header: HashMap<String, String>
}

impl Endpoint {
    pub fn new(url: String, timeout: Duration, header: HashMap<String,String>) -> Endpoint {
        Endpoint {
            url: Endpoint::url(&url),
            client: Endpoint::connector(timeout),
            header: header
        }
    }
    fn url (url: &String) -> Url {
        Url::parse(url).unwrap()
    }

    fn connector(timeout: Duration) -> hyper::Client<Handler> {
        hyper::Client::<Handler>::configure()
            .connect_timeout(timeout)
            .keep_alive(true)
            .keep_alive_timeout(Some(timeout))
            .build()
            .unwrap()
    }
}

impl Client {
    pub fn request(endpoint: &Endpoint, request: &Request) -> String
    {
        let req = request.clone();
        let client = endpoint.client.clone();
        let mut url = endpoint.url.clone();
        let (tx, rx) = mpsc::channel();

        let handler = Handler {
            request: req,
            response: None,
            sender: tx,
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) \
            AppleWebKit/537.36 (KHTML, like Gecko) \
            Chrome/52.0.2743.116 Safari/537.36".to_owned(),
        };

        let route =
        match request.route.as_ref() {
            Some(x) =>  String::new() + "/" + x,
            None => "".to_string()
        };

        url.set_path(&route);
        let _ = client.request(url, handler);

        let (_, res) = rx.recv().unwrap();
        client.close();

        println!("at response");
        let v = res.unwrap().body.unwrap();
        let body_string = String::from_utf8(v).unwrap();

        body_string
    }
}

impl hyper::client::Handler<HttpStream> for Handler {
    fn on_request(&mut self, req: &mut HyperRequest) -> Next {
        req.set_method(self.request.method.to_owned());
        req.headers_mut().set(Connection::close());
        req.headers_mut().set(UserAgent(self.user_agent.clone()));
        //req.headers_mut().set_raw("Content-Type","application/x-www-form-urlencoded");
        if self.request.body.is_some() {
            Next::write()
        } else {
            self.read()
        }
    }

    fn on_request_writable(&mut self, _encoder: &mut Encoder<HttpStream>) -> Next {
        println!("in on reuqest write");
        if let Some(ref mut body) = self.request.body {
            _encoder.write(body.as_bytes()).unwrap();
        }

        _encoder.close();
        Next::read()
    }

    fn on_response(&mut self, response: HyperResponse) -> Next {
        let status = response.status();
        let headers = response.headers();
        //debug!("Got {} for {}", status, &self.request.route.unwrap());
        self.response = Some(Response {
            status: status.clone(),
            headers: headers.clone(),
            body: None
        });
        println!("{}", status.to_string());
        match status {
            &StatusCode::Ok => {
                println!("reading");
                //if is_html(headers) {
                self.read()
                //} else {
                //    self.return_response()
                //}
            },
            &StatusCode::Created => {
                println!("reading");
                self.read()
            }
            _ => {
                println!("not reading");
                self.return_response()
            }
        }
    }

    fn on_response_readable(&mut self, decoder: &mut Decoder<HttpStream>) -> Next {
        let mut read_result = None;
        if let Some(ref mut response) = self.response {
            if response.body.is_none() {
                response.body = Some(Vec::new());
            }
            if let Some(ref mut body) = response.body {
                read_result = Some(io::copy(decoder, body));
            }
        }
        if let Some(read_result) = read_result {
            match read_result {
                Ok(0) => self.return_response(),
                Ok(_) => self.read(),
                Err(e) => match e.kind() {
                    io::ErrorKind::WouldBlock => Next::read(),
                    _ => {
                        //info!("Response read error for {}: {}", self.request.url, e);
                        self.return_response()
                    }
                }
            }
        } else {
            panic!();
        }

    }

    fn on_error(&mut self, err: hyper::Error) -> Next {
        println!("we have an error: {}", err);
        //info!("Http error for {}: {}", self.request.url, err);
        self.send_result();
        Next::remove()
    }
}





