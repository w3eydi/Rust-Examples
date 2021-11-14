#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::fs::{FileServer, TempFile, relative};
use rocket::http::{Status, ContentType};
use rocket::form::{Form, Context, Contextual, FromForm};

use rocket_dyn_templates::{Template};

pub mod schema;
mod diesel_mysql;

#[derive(Debug, FromForm)]
struct Submission<'v> {
    #[field(validate = len(1..))]
    title: &'v str,
    #[field(validate = len(1..250))]
    r#abstract: &'v str
}

#[derive(Debug, FromForm)]
struct Submit<'s> {
    submission: Submission<'s>
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}

#[post("/", data="<form>")]
fn submit<'a>(form: Form<Contextual<'a, Submit<'a>>>) -> (Status, Template) {
    let template = match form.value {
        Some(ref submission) => {
            println!("{:#?}", submission.submission.title);

            let connection = diesel_mysql::establish_connection();
            diesel_mysql::create_post(&connection, submission.submission.title,
                submission.submission.r#abstract);

            Template::render("success", &form.context)
        }
        None => Template::render("index", &form.context)
    };

    (form.context.status(), template)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
}
