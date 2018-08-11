#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate argon2;
extern crate http_02;
extern crate rand;
extern crate structopt;
#[macro_use]
extern crate warp;

use argon2::Config;
use http_02::cli::Cli;
use rand::prelude::*;
use structopt::StructOpt;
use warp::Filter;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Database definition
type Db = Arc<Mutex<HashMap<String, String>>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let _args = Cli::from_args();

  let db = Arc::new(Mutex::new(HashMap::new()));
  let db = warp::any().map(move || db.clone());

  let signup = warp::get(path!("signup"))
    .and(warp::query::<HashMap<String, String>>())
    .and(db.clone())
    .map(|query: HashMap<String, String>, db: Db| {
      let config = Config::default();
      let salt = random_bytes();

      // Not sure how to handle errors from this function, so we just unwrap on
      // user input for now.
      let username = &query.get("username").unwrap();
      let password = &query.get("password").unwrap();
      let hash =
        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();

      let mut db = db.lock().unwrap();
      db.insert(username.to_string(), hash.clone());

      format!(
        "username: {:?}\npassword: {:?}\nhash: {:?}",
        username, password, hash
      )
    });

  let login = warp::get(path!("login"))
    .and(warp::query::<HashMap<String, String>>())
    .and(db.clone())
    .map(|query: HashMap<String, String>, db: Db| {
      let username = &query.get("username").unwrap();
      let password = &query.get("password").unwrap();

      let db = db.lock().unwrap();
      let hash = db.get(*username).unwrap();

      let matches = argon2::verify_encoded(hash, password.as_bytes()).unwrap();
      format!("okay! {:?}", matches)
    });

  let log = warp::log("http_02");
  let server = warp::serve(login.or(signup).with(log));

  println!("listening on port 8080");
  server.run(([127, 0, 0, 1], 8080));
  Ok(())
}

fn random_bytes() -> Vec<u8> {
  let mut rng = rand::thread_rng();
  let mut res = vec![0; 256];
  rng.fill_bytes(&mut res);
  res
}
