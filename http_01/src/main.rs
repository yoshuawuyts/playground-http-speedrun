#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate clap_flags;
extern crate futures;
extern crate http_01;
extern crate hyper;
extern crate structopt;
extern crate tokio;
#[macro_use]
extern crate log;

use futures::prelude::*;
use http_01::cli::Cli;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  let listener = args.port.bind()?;

  args.logger.log_all(args.verbosity.log_level())?;

  let handle = tokio::reactor::Handle::current();
  let listener = tokio::net::TcpListener::from_std(listener, &handle)?;
  let addr = listener.local_addr()?;

  let server = Server::builder(listener.incoming())
    .serve(|| service_fn_ok(|_| Response::new(Body::from("Hello World"))))
    .map_err(|e| eprintln!("server error: {}", e));

  info!("Server listening on {}", addr);
  tokio::run(server);

  Ok(())
}
