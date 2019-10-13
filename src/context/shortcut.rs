use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

use rocket_contrib::databases::redis::{self, Commands};

#[database("rustly_shortcuts")]
pub struct ShortcutsDbConn(redis::Connection);

impl ShortcutsDbConn {
    pub fn fetch(&self, id: String) -> Option<Shortcut> {
        match self.hget("urls", &id) {
            Ok(value) => Some(Shortcut {id: id, destination: value}),
            Err(_e) => None,
        }
    }

    pub fn get_incr(&self) -> u64 {
        let _: () = self.incr("counter", 1).unwrap();
        self.get("counter").unwrap()
    }

    pub fn create(&self, id: &String, dest: &String) {
        let _: () = self.hset("urls", id, dest).unwrap();
    }
}

#[derive(Serialize, Debug)]
pub struct Shortcut {
    pub id: String,
    pub destination: String,
}

impl<'r> Responder<'r> for Shortcut {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .header(ContentType::new("text", "json"))
            .ok()
    }
}

#[derive(FromForm)]
pub struct NewShortcut {
    pub url: String,
}
