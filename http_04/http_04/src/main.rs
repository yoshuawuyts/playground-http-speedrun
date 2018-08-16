#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate structopt;
extern crate http_04;

use http_04::cli::Cli;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  args.logger.log_all(args.verbosity.log_level())?;
  info!("program started");
  Ok(())
}
