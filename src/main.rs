#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::sync::Mutex;
use std::collections::HashMap;
use rocket::State;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use url::Url;

type DB = Mutex<HashMap<String, String>>;

#[get("/<id>")]
fn redirect(id: String, state: State<DB>) -> Redirect {
    let db = state.lock().expect("Unable to lock");
    match db.get(&id) {
        Some(url) => Redirect::to(url.clone()),
        None => Redirect::to("/")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    slug: Option<String>,
    url: String,
}

fn generate_slug() -> String {
    let result = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .collect();
    result
}

#[post("/", format = "application/json", data = "<submission>")]
fn shorten(submission: Json<Submission>, state: State<DB>) -> JsonValue {
    // Don't know why I can't destructure this, should be able to
    // let Submission { slug: s, url: u } = submission;
    let mut db = state.lock().expect("Unable to lock");
    let slug = match &submission.slug {
        Some(s) => s.clone(),
        None => generate_slug(), //TODO: Replace with slug generator.
    };

    let url = match Url::parse(&submission.url) {
        Ok(u) => u.into_string(),
        Err(e) => return json!({"status": "error", "reason": format!("Couldn't shorten, url is funky. ({})", e)}),
    };

    if db.contains_key(&slug) {
        json!({"status": "error", "reason": "Couldn't shorten, url already points somewhere."})
    } else {
        db.insert(slug.clone(), url);
        json!({"status": "success", "slug": slug})
    }
}

fn main() {
    let db = Mutex::new(HashMap::<String, String>::new());
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/build")))
        .mount("/", routes![redirect, shorten])
        .manage(db)
        .launch();
}