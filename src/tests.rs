use rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn test_xpto() {
    let client = Client::new(super::server()).expect("valid rocket instance");
    let mut response = client.get("/api").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Welcome to the API".into()));
}