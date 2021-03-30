use rocket::request::Form;
use rocket::response::content::Json;
use crate::api::models::user::User;

#[get("/users")]
pub fn get_users() -> Json<&'static str> {
    Json("{
        'status': 'success',
        'message': 'No users!'
      }")
}

#[post("/users", data = "<user_form>")]
fn post_user(user_form: Form<User>) -> Json<&'static str> {
    Json("{
        'status': 'success',
        'message': 'USER'
      }")
}