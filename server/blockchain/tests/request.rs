#[path = "../src/request.rs"]
mod request;

#[test]
fn handle_method_returns_get() {
    let s = "GET / HTTP/1.1";
    match request::handle_method(s) {
        request::Methods::Get => (),
        _ => panic!("Expected GET"),
    }
}

#[test]
fn handle_method_returns_post() {
    let s = "POST / HTTP/1.1";
    match request::handle_method(s) {
        request::Methods::Post => (),
        _ => panic!("Expected POST"),
    }
}

#[test]
fn get_route_returns_root() {
    let s = "GET / HTTP/1.1";
    assert_eq!(request::get_route(s), "/");
}

#[test]
fn get_route_returns_ping() {
    let s = "GET /ping HTTP/1.1";
    assert_eq!(request::get_route(s), "/ping");
}

#[test]
fn get_route_ignores_params() {
    let s = "POST /ping?foo=bar HTTP/1.1";
    assert_eq!(request::get_route(s), "/ping");
}

#[test]
fn get_query_params_returns_none() {
    let s = "GET /ping HTTP/1.1";
    assert_eq!(request::get_query_params::<()>(s), None);
}

#[test]
fn get_query_params_returns_params() {
    let s = "GET /ping?foo=bar HTTP/1.1";
    assert_eq!(
        request::get_query_params(s),
        Some(serde_json::json!({ "foo": "bar" }))
    );
}

#[test]
fn get_query_params_returns_params_with_multiple_values() {
    let s = "GET /ping?foo=bar&num=1 HTTP/1.1";
    assert_eq!(
        request::get_query_params(s),
        Some(serde_json::json!({ "foo": "bar", "num": 1 }))
    );
}

#[test]
fn get_body_returns_none() {
    let s = "GET /ping HTTP/1.1".to_string();
    assert_eq!(request::get_body(s), None);
}

#[test]
fn get_body_returns_body() {
    let s = "POST /ping HTTP/1.1\r\n\r\n{\n\r\"foo\": \"bar\"\n\r}".to_string();
    assert_eq!(
        request::get_body(s),
        Some(String::from("{\n\r\"foo\": \"bar\"\n\r}"))
    );
}

#[test]
fn deserialize_post_stake_returns_post_stake() {
    let s = "{\"stake\": 1}".to_string();
    let request = request::deserialize::<request::PostStake>(&s);
    assert_eq!(request.stake, 1);
}

#[test]
fn create_json_from_query_returns_json() {
    let s = "foo=bar&num=1";
    assert_eq!(
        request::create_json_from_query(s),
        "{ \"foo\": \"bar\", \"num\": 1 }"
    );
}

#[test]
fn parse_request_header_returns_get() {
    let s = "GET / HTTP/1.1".to_string();
    if let Ok(request_header) = request::parse_request_header(s) {
        match request_header {
            request::Request::Get => (),
            _ => panic!("Expected GET"),
        }
    } else {
        panic!("Expected Ok");
    }
}

#[test]
fn parse_request_header_returns_post() {
    let s = "POST /task HTTP/1.1\r\n\r\n{\"task\": \"Test\"}".to_string();
    if let Ok(request_header) = request::parse_request_header(s) {
        match request_header {
            request::Request::PostTask(_) => (),
            _ => panic!("Expected POST"),
        }
    } else {
        panic!("Expected Ok");
    }
}
