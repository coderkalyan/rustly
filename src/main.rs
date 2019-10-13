#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod context;

use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;

use context::shortcut::{Shortcut, ShortcutsDbConn, NewShortcut};
use context::shortener::Shortener;

#[get("/api/shortcuts/<id>")]
fn lookup(conn: ShortcutsDbConn, id: String) -> Option<Shortcut> {
    conn.fetch(id)
}

#[get("/<id>")]
fn redirect(conn: ShortcutsDbConn, id: String) -> Option<Redirect> {
    match conn.fetch(id) {
        Some(v) => Some(Redirect::to(v.destination)),
        None => None,
    }
}

#[post("/api/shortcuts/new", data = "<new_shortcut>")]
fn create(conn: ShortcutsDbConn, shortener: State<Shortener>,
          new_shortcut: Form<NewShortcut>) -> Shortcut {
    let id = conn.get_incr();
    let hash = shortener.hash(id);
    conn.create(&hash, &new_shortcut.url);
    Shortcut {
        id: hash.clone(),
        destination: new_shortcut.url.clone(),
    }
}

fn main() {
    rocket::ignite()
        .attach(ShortcutsDbConn::fairing())
        .manage(Shortener::new())
        .mount(
            "/",
            rocket::routes![lookup, redirect, create]
            )
        .launch();
}
