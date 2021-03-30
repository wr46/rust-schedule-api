use diesel::Queryable;

#[derive(Queryable, FromForm, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
  }