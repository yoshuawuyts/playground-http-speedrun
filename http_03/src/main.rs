#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![feature(rust_2018_preview, uniform_paths)]

#[macro_use]
extern crate log;

use futures::prelude::*;
use http_03::cli::Cli;
use hyper::{service::service_fn_ok, Body, Response, Server};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  args.logger.log_all(args.verbosity.log_level())?;
  let service = || service_fn_ok(|_| Response::new(Body::from("Hello World")));
  let server = Server::from_tcp(args.port.bind()?)?.serve(service);
  info!("listening on {}", server.local_addr());
  tokio::run(server.map_err(|err| error!("server error {}", err)));
  Ok(())
}
