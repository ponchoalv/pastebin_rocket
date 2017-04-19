#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]


extern crate rocket;
extern crate rand;
extern crate uuid;
extern crate rocket_contrib;

mod paste_id;

use paste_id::PasteID;

use std::io;
use std::path::Path;
use std::path::MAIN_SEPARATOR;
use std::fs::File;
use std::str::FromStr;

use rocket::Data;
use uuid::Uuid;
use rocket_contrib::UUID;


#[derive(FromForm)]
struct UserQuery {
    id: UUID
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
      POST /
          accepts raw data in the body of the request and responds with a URL of a page containing the body's content

      GET /<id>
          retrieves the content for the paste with id <id>
"
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    let id = PasteID::new(3);
    let filename = format!("upload{sep}{id}", sep = MAIN_SEPARATOR, id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<File> {
    let filename = format!("upload{sep}{id}", sep = MAIN_SEPARATOR, id = id);
    File::open(&filename).ok()
}

#[get("/uuid")]
fn uuid() -> String {
    let my_uuid = Uuid::new_v4();
    format!("El UUID generado es: {uuid}", uuid = my_uuid)
}

#[get("/uuid?<user_query>")]
fn user(user_query: UserQuery) -> String {
    let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
    format!("uuid is equal c1aa1e3b-9614-4895-9ebd-705255fa5bc2 ==> {:?}", (user_query.id == Uuid::from_str(uuid_str).unwrap()))
}

fn main() {
    rocket::ignite().mount("/", routes![index, upload, retrieve, uuid, user]).launch()
}
