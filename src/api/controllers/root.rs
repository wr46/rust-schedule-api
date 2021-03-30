use rocket::Request;

#[get("/a")]
pub fn get_api() -> &'static str {
    "Welcome to the API"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}