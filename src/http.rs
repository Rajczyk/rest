
use std::io;
use std::sync::mpsc::{Sender};
use std::sync::mpsc;
use std::time::Duration;

use hyper::client::{Client as HyperClient, Request as HyperRequest, Response as HyperResponse, DefaultTransport as HttpStream};
use hyper::header::{Connection, Headers, UserAgent};
use hyper::{Decoder, Encoder, Next};
use hyper::Method::{Get};
use hyper::status::StatusCode;
use hyper;

use url::Url;


pub type ResultSender = mpsc::Sender<(Request, Option<Response>)>;

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
    //timeout: u64,
    //user_agent: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub url: Url
}

impl Request {
    pub fn new(url: Url) -> Self {
        Request { url: url }
    }

    pub fn from_str(url: &str) -> Self {
        Request::new(url.parse().unwrap())
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

pub struct Client
{

}

impl Client {

    pub fn new() -> Client {
        Client {}
    }

    pub fn request(&self, url: &str) -> String
    {
        println!("hyper code");
        let mut url = Url::parse("http://jsonplaceholder.typicode.com/posts/1").unwrap();

        let client = hyper::Client::new().unwrap();
        let (tx, rx) = mpsc::channel();


        let req2 = Request {
            url : url.clone()
        };

        let handler = Handler {
            request: req2,
            response: None,
            sender: tx,
            //timeout: timeout,
            //user_agent: user_agent.to_owned(),
        };

        let r  = client.request(url, handler);

        println!("Waiting for response");
        let (re, res) = rx.recv().unwrap();
        println!("Got for response");

        client.close();


        let v = res.unwrap().body.unwrap();

        println!("Printing Response Next:");
        println!("{}", String::from_utf8(v).unwrap());
        //println!("Headers:\n{}", handler.response.headers());

        String::new()

    }
}

impl hyper::client::Handler<HttpStream> for Handler {
    fn on_request(&mut self, req: &mut HyperRequest) -> Next {
        let mut headers = req.headers_mut();
        headers.set(Connection::close());
        //headers.set(UserAgent(self.user_agent.clone()));
        self.read()
    }

    fn on_request_writable(&mut self, _encoder: &mut Encoder<HttpStream>) -> Next {
        self.read()
    }

    fn on_response(&mut self, response: HyperResponse) -> Next {
        let status = response.status();
        let headers = response.headers();
        //debug!("Got {} for {}", status, self.request.url);
        self.response = Some(Response {
            status: status.clone(),
            headers: headers.clone(),
            body: None
        });
        match status {
            &StatusCode::Ok => {
                //if is_html(headers) {
                self.read()
                //} else {
                //    self.return_response()
                //}
            },
            _ => self.return_response()
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
        //info!("Http error for {}: {}", self.request.url, err);
        self.send_result();
        Next::remove()
    }
}





