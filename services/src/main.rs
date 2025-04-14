// curl http://127.0.0.1:8000/hello/foo/21
#[rocket::get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![hello])
}
