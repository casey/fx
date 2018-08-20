use common::*;

pub fn find_user_fxfile() -> Result<PathBuf, Error> {
  let user_fxfile_paths = Platform::user_fxfile_paths()?;

  let mut found = Vec::new();

  for path in &user_fxfile_paths {
    match path.metadata() {
      Ok(metadata) => {
        if metadata.is_file() {
          found.push(path.clone());
        }
      }
      Err(io_error) => {
        if io_error.kind() != io::ErrorKind::NotFound {
          return Err(Error::FxfileIo {
            io_error,
            path: path.clone(),
          });
        }
      }
    }
  }

  if found.is_empty() {
    Err(Error::UserFxfileNotFound {
      searched_paths: user_fxfile_paths,
    })
  } else if found.len() > 1 {
    Err(Error::MultipleUserFxfilesFound { found })
  } else {
    Ok(found.into_iter().next().unwrap())
  }
}
