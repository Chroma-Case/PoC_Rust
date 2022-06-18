use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::request::{self, Request};
use rocket::serde::{json::Json, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    name: String,
    age: u8,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Error {
    InvalidAge,
    InvalidName,
}

#[get("/")]
pub fn index() -> Json<Vec<User>> {
    Json(vec![
        User {
            name: "test".to_string(),
            age: 3,
        },
        User {
            name: "pierre".to_string(),
            age: 67,
        },
    ])
}

#[get("/<uid>", format = "json")]
pub fn find_by_uid(uid: u64) -> Json<Option<User>> {
    if uid == 3 {
        return Json(None);
    }
    Json(Some(User {
        name: "bababouie".to_string(),
        age: 246,
    }))
}

#[post("/", format = "json", data = "<user>")]
pub fn post(user: Json<User>) -> Json<Result<User, Error>> {
    if user.age == 0 {
        return Json(Err(Error::InvalidAge));
    }
    Json(Ok(user.into_inner()))
}
