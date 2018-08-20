use common::*;

pub fn run() -> Result<(), Vec<Error>> {
  let user_fxfile_path = find_user_fxfile()?;

  let fx = Fx::from_file(user_fxfile_path)?;

  let capture = capture()?;

  let matches = fx.matches(&capture);

  if matches.is_empty() {
    Err(Error::NoMatches)?;
  }

  let selections = select(&matches)?;

  for selection in selections {
    println!("{}", selection);
  }

  Ok(())
}
