#[macro_use] extern crate rocket;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct BasicJson(&'static str);

#[get("/")]
fn index() -> BasicJson {
    BasicJson("{\"version\": \"blorg\"}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index])
}
