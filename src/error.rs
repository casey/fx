use common::*;

use regex;
use Error::*;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

pub enum Error {
  UserFxfileBasePathEnvironmentVariableUnset {
    variable: &'static str,
  },
  UserFxfileNotFound {
    searched_paths: Vec<PathBuf>,
  },
  MultipleUserFxfilesFound {
    found: Vec<PathBuf>,
  },
  FxfileIo {
    io_error: io::Error,
    path: PathBuf,
  },
  FxfileNotUtf8 {
    from_utf8_error: FromUtf8Error,
    path: PathBuf,
  },
  FxfilePattern {
    regex_error: regex::Error,
    path: PathBuf,
    line_index: usize,
  },
  FxfileEmpty {
    path: PathBuf,
  },
  CaptureExecution {
    io_error: io::Error,
    command: &'static [&'static str],
  },
  CaptureStatus {
    status: ExitStatus,
    stderr: String,
    stdout: String,
    command: &'static [&'static str],
  },
  NoMatches,
  UnmatchedSelections {
    unmatched: Vec<String>,
    matches: Vec<String>,
  },
  Internal {
    message: String,
  },
}

impl Error {
  fn fmt_description(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      UserFxfileBasePathEnvironmentVariableUnset { variable } => write!(
        f,
        "unable to search for user fxfile, environment variable `{}` unset was",
        variable
      ),
      UserFxfileNotFound { searched_paths } => {
        writeln!(f, "user fxfile not found, paths searched:")?;
        for (i, path) in searched_paths.iter().enumerate() {
          write!(f, "\t{}", path.display())?;
          if i != searched_paths.len() - 1 {
            writeln!(f)?;
          }
        }
        Ok(())
      }
      MultipleUserFxfilesFound { found } => {
        writeln!(f, "found multiple user fxfiles:")?;
        for (i, path) in found.iter().enumerate() {
          writeln!(f, "\t{}", path.display())?;
          if i != found.len() - 1 {
            writeln!(f)?;
          }
        }
        Ok(())
      }
      FxfileIo { io_error, .. } => write!(f, "I/O error reading fxfile: {}", io_error),
      FxfileNotUtf8 {
        from_utf8_error, ..
      } => write!(f, "fxfile not valid UTF-8: {}", from_utf8_error),
      FxfilePattern { regex_error, .. } => write!(f, "bad pattern: {}", regex_error),
      FxfileEmpty { .. } => write!(f, "fxfile contained no patterns"),
      CaptureExecution { io_error, .. } => {
        write!(f, "could not execute capture command: {}", io_error)
      }
      CaptureStatus { status, .. } => {
        #[cfg(unix)]
        let signal = status.signal();

        #[cfg(not(unix))]
        let signal = None;

        if let Some(signal) = signal {
          write!(f, "capture failed with signal: `{}`", signal)?;
        } else if let Some(code) = status.code() {
          write!(f, "capture failed with code: `{}`", code)?;
        } else {
          write!(f, "capture failed with unknown code")?;
        }

        Ok(())
      }
      NoMatches => write!(f, "found no matching patterns"),
      UnmatchedSelections {
        unmatched, matches
      } => {
        write!(f, "selections returned which were not present in capture:")?;
        for unmatched in unmatched {
          write!(f, " `{}`", unmatched)?;
        }
        writeln!(f)?;
        writeln!(f, "matches in capture: {}", matches.join(", "))
      },
      Internal{message} => write!(f, "internal error, this is a bug: {} (consider filing an issue: https://github.com/casey/fx/issues/new)", message),
    }
  }

  fn location(&self) -> Option<Location> {
    use Location::*;
    match self {
      FxfileIo { path, .. } => Some(Path { path: &path }),
      FxfileNotUtf8 { path, .. } => Some(Path { path: &path }),
      FxfilePattern {
        path, line_index, ..
      } => Some(Line {
        path: &path,
        line_index: *line_index,
      }),
      FxfileEmpty { path } => Some(Path { path: &path }),
      _ => None,
    }
  }

  fn fmt_coda(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      CaptureExecution { command, .. } => write!(f, "capture command: `{}`", command.join(" ")),
      CaptureStatus {
        command,
        stderr,
        stdout,
        ..
      } => {
        writeln!(f, "capture command: `{}`", command.join(" "))?;
        writeln!(f, "capture stdout:\n{}", stdout)?;
        writeln!(f, "capture stderr:\n{}", stderr)
      }
      _ => Ok(()),
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "error: ")?;
    self.fmt_description(f)?;
    writeln!(f)?;
    if let Some(location) = self.location() {
      writeln!(f, "{}", location)?;
    }
    self.fmt_coda(f)
  }
}

impl From<Error> for Vec<Error> {
  fn from(error: Error) -> Vec<Error> {
    vec![error]
  }
}
