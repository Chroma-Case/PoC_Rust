#[macro_use] extern crate rocket;
mod user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/user", routes![user::index, user::find_by_uid, user::post])
}