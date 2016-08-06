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
        let result = response.unwrap();
        assert_eq!(result.is_empty(), false);

        let expected_result =
r#"{
  "userId": 1,
  "id": 1,
  "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
  "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
}"#.to_string();

        assert_eq!(&result, &expected_result);
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
        let result = response.unwrap();
        assert_eq!(result.is_empty(), false);

        let expected_result =
r#"[
  {
    "userId": 1,
    "id": 1,
    "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
    "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
  },
  {
    "userId": 1,
    "id": 2,
    "title": "qui est esse",
    "body": "est rerum tempore vitae\nsequi sint nihil reprehenderit dolor beatae ea dolores neque\nfugiat blanditiis voluptate porro vel nihil molestiae ut reiciendis\nqui aperiam non debitis possimus qui neque nisi nulla"
  },
  {
    "userId": 1,
    "id": 3,
    "title": "ea molestias quasi exercitationem repellat qui ipsa sit aut",
    "body": "et iusto sed quo iure\nvoluptatem occaecati omnis eligendi aut ad\nvoluptatem doloribus vel accusantium quis pariatur\nmolestiae porro eius odio et labore et velit aut"
  },
  {
    "userId": 1,
    "id": 4,
    "title": "eum et est occaecati",
    "body": "ullam et saepe reiciendis voluptatem adipisci\nsit amet autem assumenda provident rerum culpa\nquis hic commodi nesciunt rem tenetur doloremque ipsam iure\nquis sunt voluptatem rerum illo velit"
  },
  {
    "userId": 1,
    "id": 5,
    "title": "nesciunt quas odio",
    "body": "repudiandae veniam quaerat sunt sed\nalias aut fugiat sit autem sed est\nvoluptatem omnis possimus esse voluptatibus quis\nest aut tenetur dolor neque"
  },
  {
    "userId": 1,
    "id": 6,
    "title": "dolorem eum magni eos aperiam quia",
    "body": "ut aspernatur corporis harum nihil quis provident sequi\nmollitia nobis aliquid molestiae\nperspiciatis et ea nemo ab reprehenderit accusantium quas\nvoluptate dolores velit et doloremque molestiae"
  },
  {
    "userId": 1,
    "id": 7,
    "title": "magnam facilis autem",
    "body": "dolore placeat quibusdam ea quo vitae\nmagni quis enim qui quis quo nemo aut saepe\nquidem repellat excepturi ut quia\nsunt ut sequi eos ea sed quas"
  },
  {
    "userId": 1,
    "id": 8,
    "title": "dolorem dolore est ipsam",
    "body": "dignissimos aperiam dolorem qui eum\nfacilis quibusdam animi sint suscipit qui sint possimus cum\nquaerat magni maiores excepturi\nipsam ut commodi dolor voluptatum modi aut vitae"
  },
  {
    "userId": 1,
    "id": 9,
    "title": "nesciunt iure omnis dolorem tempora et accusantium",
    "body": "consectetur animi nesciunt iure dolore\nenim quia ad\nveniam autem ut quam aut nobis\net est aut quod aut provident voluptas autem voluptas"
  },
  {
    "userId": 1,
    "id": 10,
    "title": "optio molestias id quia eum",
    "body": "quo et expedita modi cum officia vel magni\ndoloribus qui repudiandae\nvero nisi sit\nquos veniam quod sed accusamus veritatis error"
  }
]"#;
        assert_eq!(&result, &expected_result);
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
        let result = response.unwrap();
        assert_eq!(result.is_empty(), false);

        let expected_result = 27520;

        //this is too big a string to include for an exact match,
        //so we will just check the length
        assert_eq!(result.len(), expected_result);
    }

    #[test]
    fn rest_post_single_add_parameter() {
        let api = rest::Endpoint::configure()
            .url(URL)
            .timeout(Duration::from_secs(5))
            .build();

        let request = rest::Request::post()
            .path("post")
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