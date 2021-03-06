#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate human_panic;
extern crate memdb;
#[macro_use]
extern crate serde_derive;
extern crate secure_password;
extern crate serde_qs;
extern crate structopt;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate http_04;
extern crate hyper;
extern crate tokio;

use failure::Error;
use futures::prelude::*;
use http_04::cli::Cli;
use hyper::{service::service_fn, Body, Request, Response, Server};
use memdb::Memdb;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  setup_panic!();
  let args = Cli::from_args();
  args.logger.log_all(args.verbosity.log_level())?;
  let db = Memdb::<String, Vec<u8>>::new();

  let service = move || {
    let db = db.clone();

    service_fn(
      move |req: Request<Body>| -> Result<Response<Body>, String> {
        let mut db = db.clone();
        info!("route {}", req.uri().path());

        match req.uri().path() {
          "/signup" => Ok(signup(&req, &mut db).unwrap()),
          "/login" => Ok(login(&req, &mut db).unwrap()),
          _ => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
        }
      },
    )
  };

  let server = Server::from_tcp(args.port.bind()?)?.serve(service);
  info!("listening on {}", server.local_addr());
  tokio::run(server.map_err(|err| error!("server error {}", err)));
  Ok(())
}

#[derive(Debug, PartialEq, Deserialize)]
struct Credentials {
  username: String,
  password: String,
}

fn signup(
  req: &Request<Body>,
  db: &mut Memdb<String, Vec<u8>>,
) -> Result<Response<Body>, Error> {
  let query = match req.uri().query() {
    Some(query) => query,
    None => bail!("No query provided"),
  };
  let creds: Credentials = serde_qs::from_str(query).unwrap();
  let hash = secure_password::hash(&creds.password.as_bytes())?;
  debug!("Created account for {}", &creds.username);
  db.set(creds.username, hash);
  Ok(Response::default())
}

fn login(
  req: &Request<Body>,
  db: &mut Memdb<String, Vec<u8>>,
) -> Result<Response<Body>, Error> {
  let query = match req.uri().query() {
    Some(query) => query,
    None => bail!("No query provided"),
  };
  let creds: Credentials = serde_qs::from_str(query).unwrap();
  let pass = &creds.password.as_bytes();
  let hash = match db.get(creds.username.clone()) {
    Some(hash) => hash,
    None => bail!("Username not found"),
  };
  ensure!(secure_password::verify(pass, &hash)?, "Failed to login");
  debug!("Signed in {}", &creds.username);
  Ok(Response::default())
}
