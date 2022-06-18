use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::request::{self, Request};
use rocket::serde::json::json;
use rocket::serde::{json::Json, json::Value, Deserialize, Serialize};

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
fn index() -> Json<Vec<User>> {
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
fn find_by_uid(uid: u64) -> Json<Option<User>> {
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

#[put("/", format = "json", data = "<user>")]
fn put(user: Json<User>) -> Json<Result<User, Error>> {
    if user.age == 0 {
        return Json(Err(Error::InvalidAge));
    }
    Json(Ok(user.into_inner()))
}

#[delete("/<uid>", format = "json")]
fn delete(uid: u64) -> Json<Option<User>> {
    if uid == 3 {
        return Json(None);
    }
    Json(Some(User {
        name: "bababouie".to_string(),
        age: 246,
    }))
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("User", |rocket| async {
        rocket
            .mount("/user", routes![index, find_by_uid, post, put, delete])
            .register("/user", catchers![not_found])
    })
}
