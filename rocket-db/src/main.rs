#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::form::{Context};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    /*Template::render("index", context!{
        title: "Hello"
    })*/
    Template::render("index", &Context::default())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
}
