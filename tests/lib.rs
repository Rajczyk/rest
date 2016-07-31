extern crate gj;
extern crate simple-rest;

#[cfg(test)]
mod tests {

    use std::time::Duration;

    #[test]
    fn rest_get_single() {
        let api = RestBuilder::new()
            .api("http://jsonplaceholder.typicode.com")
            .build();

        let x = api.query("")
            .timeout(Duration::from_secs(5))
            .get();
    }


}