quick_error! {
    /// Error types.
    #[derive(Debug)]
    pub enum Error {
    /// Wrapper for a `::std::io::Error`
        IoError(err: ::std::io::Error) {
        description("IO error")
        display("IO error: {}", err)
        cause(err)
        from()
        }
    /// Wrapper for a `::url::ParseError`
        UrlParseError(err: ::url::ParseError) {
        description("Url Parse error")
        display("Url Parse error: {}", err)
        cause(err)
        from()
        }
    /// Wrapper for a `::hyper::Error`
        HttpError(err: ::hyper::Error) {
        description("http error")
        display("http error: {}", err)
        cause(err)
        from()
        }
    }
}