extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate time;



pub mod Rest {


    use std::io;
    use std::sync::mpsc::{Sender};
    use std::sync::mpsc;
    use std::time::Duration;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    use hyper::client::{Client as HyperClient, Request as HyperRequest, Response as HyperResponse, DefaultTransport as HttpStream};
    use hyper::header::{Connection, Headers, UserAgent};
    use hyper::{Decoder, Encoder, Next};
    use hyper::Method::{Get};
    use hyper::status::StatusCode;
    use hyper;


    use url::Url;

    use serde_json::Value;
    use serde_json::builder::{ArrayBuilder, ObjectBuilder};

    use std::io::Error as IoError;
    use hyper::Error as HttpError;
    use url::ParseError as UrlParseError;

    #[derive(Debug)]
    pub enum Error {
        UrlParseError(UrlParseError),
        HttpRequestError(HttpError),
        HttpIoError(IoError)
    }

    pub struct Response {
        code: u16,
        status: StatusCode,
        headers: Headers,
        pub body: String,
    }

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


        pub fn execute(endpoint: Endpoint, request: Request) -> Result<Response, Error> {


            println!("hyper code");
            let mut url = Url::parse("http://jsonplaceholder.typicode.com/posts/1").unwrap();

            let client = hyper::Client::new().unwrap();
            let (tx, rx) = mpsc::channel();

            let handler = Handler {
                request: request,
                response: None,
                sender: tx,
                timeout: timeout,
                //user_agent: user_agent.to_owned(),
            };

            let r  = client.request(url, handler);

            let _ = rx.recv();

            client.close();



            println!("Response: {}", handler.response.status());
            println!("Headers:\n{}", handler.response.headers());

            Ok(Response {
                code: 1,
                status: StatusCode::Accepted,
                headers: Headers::default(),
                body: "".to_string()
            })
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
            RequestBuilder { query: String::new()  }
        }


    }


    pub struct Handler {
        request: Request,
        response: Option<Response>,
        sender: ResultSender,
        timeout: u64,
        //user_agent: String,
    }

    impl Handler {
        fn read(&self) -> Next {
            Next::read().timeout(Duration::from_secs(self.timeout))
        }

        fn return_response(&self) -> Next {
            self.send_result();
            Next::end()
        }

        fn send_result(&self) {
            self.sender.send((self.request.clone(), self.response.clone())).unwrap();
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
                    if is_html(headers) {
                        self.read()
                    } else {
                        self.return_response()
                    }
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
                    // TODO - check that this really appends data, not overrides
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

    #[derive(Debug)]
    struct Dump(Sender<()>);

    impl Drop for Dump {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }

    fn read() -> Next {
        Next::read().timeout(Duration::from_secs(10))
    }

}

