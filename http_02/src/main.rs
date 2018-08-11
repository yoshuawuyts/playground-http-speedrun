#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate argon2;
extern crate http_02;
extern crate structopt;
#[macro_use]
extern crate warp;

use http_02::cli::Cli;
use structopt::StructOpt;
use warp::Filter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let _args = Cli::from_args();

  let login = warp::get(path!("login"))
    .and(warp::filters::query::query())
    .map(|query: String| format!("login: {}", query));

  let signup = warp::get(path!("signup")).map(|| format!("Signing up"));

  let server = warp::serve(login.or(signup));
  println!("listening on port 8080");
  server.run(([127, 0, 0, 1], 8080));
  Ok(())
}
