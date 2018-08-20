extern crate regex;

mod capture;
mod common;
mod error;
mod find_user_fxfile;
mod fx;
mod location;
mod platform;
mod run;
mod select;

use common::*;

fn main() {
  if let Err(errors) = run::run() {
    if errors.is_empty() {
      eprint!(
        "{}",
        Error::Internal {
          message: "run failed but returned no errors".to_string()
        }
      );
      process::exit(1);
    }

    for error in errors {
      eprint!("{}", error);
    }
    process::exit(1);
  }
}
