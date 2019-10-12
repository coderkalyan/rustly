#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

use rocket_contrib::databases::redis::{self, Commands};

#[database("rustly_shortcuts")]
struct ShortcutsDbConn(redis::Connection);

struct Shortcut {
    id: String,
    destination: String,
}

impl<'r> Responder<'r> for Shortcut {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}", self.destination)))
            .header(ContentType::new("text", "plain"))
            .ok()
    }
}

#[get("/api/shortcuts/<id>")]
fn lookup(conn: ShortcutsDbConn, id: String) -> Option<Shortcut> {
    match conn.hget("urls", &id) {
        Ok(value) => Some(Shortcut {id: id, destination: value}),
        Err(_e) => None,
    }
}

#[get("/<id>")]
fn redirect(id: String) -> String {
    id
}

#[post("/api/shortcuts/new/<dest>")]
fn create(dest: String) -> String {
    dest
}

fn main() {
    rocket::ignite()
        .attach(ShortcutsDbConn::fairing())
        .mount(
            "/",
            rocket::routes![lookup, redirect, create]
            )
        .launch();
}
