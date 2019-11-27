#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod context;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

use rocket::State;
use rocket::request::Form;
use rocket::response::{Redirect, NamedFile};

use rocket_contrib::templates::Template;
//use rocket_contrib::serve::StaticFiles;

use context::shortcut::{Shortcut, ShortcutsDbConn, NewShortcut};
use context::shortener::Shortener;

#[get("/")]
fn root() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert("base".to_string(), 
                   env!("RUSTLY_URL").to_string());
    Template::render("index", context)
}

#[get("/<id>")]
fn redirect(conn: ShortcutsDbConn, id: String) -> Option<Redirect> {
    match conn.fetch(id) {
        Some(v) => Some(Redirect::to(v.destination)),
        None => None,
    }
}

#[get("/api/shortcuts/<id>")]
fn lookup(conn: ShortcutsDbConn, id: String) -> Option<Shortcut> {
    conn.fetch(id)
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

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .attach(ShortcutsDbConn::fairing())
        .attach(Template::fairing())
        .manage(Shortener::new())
        .mount("/", rocket::routes![root, redirect, lookup, create, files])
        .launch();
}
