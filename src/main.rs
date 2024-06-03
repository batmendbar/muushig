#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Muushig!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
