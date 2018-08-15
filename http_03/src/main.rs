#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate http_03;
#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
extern crate structopt;
extern crate tokio;

use futures::prelude::*;
use http_03::cli::Cli;
use hyper::{service::service_fn_ok, Body, Response, Server};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  args.logger.log_all(args.verbosity.log_level())?;

  let listener = args.port.bind()?;
  let addr = listener.local_addr()?;

  let server = Server::from_tcp(listener)?
    .serve(|| service_fn_ok(|_| Response::new(Body::from("Hello World"))))
    .map_err(|err| error!("server error {}", err));

  info!("listening on {}", addr);
  tokio::run(server);
  Ok(())
}
