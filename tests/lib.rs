extern crate rest;

#[cfg(test)]
mod tests {

    use rest;
    use std::time::Duration;

    static URL : &'static str = "http://jsonplaceholder.typicode.com";

    #[test]
    fn rest_get_single_url_segment() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .add_header("Accept-Encoding", "Accept-Encoding: gzip, deflate")
            .build();

        let request = rest::Request::get()
            .path("posts/{id}")
            .add_urlsegment("id", "1")
            .build();

        let response = rest::Client::execute(api, request);
        println!("{}",response.unwrap());
        //assert!(response.unwrap(), true);
    }

    #[test]
    fn rest_get_single_add_parameter() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .build();

        let request = rest::Request::get()
            .path("posts")
            .add_parameter("userId", "1")
            .build();

        let response = rest::Client::execute(api, request);
        println!("{}",response.unwrap());
        //assert!(response.unwrap().is_empty(), false);
    }

    #[test]
    fn rest_get_list_url_segment() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .build();

        let request = rest::Request::get()
            .path("posts")
            .build();

        let response = rest::Client::execute(api, request);
        println!("{}",response.unwrap());
        //assert!(response.unwrap().is_empty(), false);
    }

    #[test]
    fn rest_post_single_add_parameter() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .build();

        let request = rest::Request::post()
            .path("posts")
            .add_parameter("title", "foo")
            .add_parameter("body", "bar")
            .add_parameter("userId", "1")
            .build();

        let response = rest::Client::execute(api, request);
        println!("{}",response.unwrap());
        //assert!(response.unwrap().is_empty(), false);
    }

    #[test]
    fn rest_put_single_add_parameter() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .build();

        let request = rest::Request::put()
            .path("posts")
            .add_parameter("title", "bar")
            .add_parameter("body", "foo")
            .add_parameter("userId", "1")
            .build();

        let response = rest::Client::execute(api, request);
        println!("{}",response.unwrap());
        //assert!(response.unwrap().is_empty(), false);
    }

    #[test]
    fn rest_patch_single_add_parameter() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .build();

        let request = rest::Request::patch()
            .path("posts/{id}")
            .add_urlsegment("id", "1")
            .add_parameter("title", "foo")
            .build();

        let response = rest::Client::execute(api, request);
        println!("{}",response.unwrap());
        //assert!(response.unwrap().is_empty(), false);
    }

    #[test]
    fn rest_delete_single_url_segment() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .build();

        let request = rest::Request::delete()
            .path("posts/{id}")
            .add_urlsegment("id", "1")
            .build();

        let response = rest::Client::execute(api, request);
        println!("{}",response.unwrap());

    }
}