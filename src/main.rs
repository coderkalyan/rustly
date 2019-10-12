#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/api/shortcuts/<id>")]
fn lookup(id: String) -> String {
    id
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
        .mount(
            "/",
            rocket::routes![lookup, redirect, create]
            )
        .launch();
}
