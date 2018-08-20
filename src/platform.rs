use common::*;

use std::env;

pub struct Platform;

pub trait PlatformInterface {
  fn user_fxfile_paths() -> Result<Vec<PathBuf>, Error>;
}

#[cfg(target_os = "windows")]
impl PlatformInterface for Platform {
  fn user_fxfile_paths() -> Result<Vec<PathBuf>, Error> {
    const VARIABLE: &str = "APPDATA";
    let base = env::var_os(VARIABLE)
      .ok_or_else(|| Error::UserFxfileBasePathEnvironmentVariableUnset { variable: VARIABLE })?;
    let mut path = PathBuf::from(base);
    path.push("fx");
    path.push("fxfile");
    Ok(vec![path])
  }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
impl PlatformInterface for Platform {
  fn user_fxfile_paths() -> Result<Vec<PathBuf>, Error> {
    const VARIABLE: &str = "HOME";
    let home = PathBuf::from(
      env::var_os(VARIABLE)
        .ok_or_else(|| Error::UserFxfileBasePathEnvironmentVariableUnset { variable: VARIABLE })?,
    );

    let mut paths = Vec::new();

    paths.push(home.join(".fxfile"));

    {
      let mut path = home.join(".config");
      path.push("fx");
      path.push("fxfile");
      paths.push(path);
    }

    if let Some(xdg_config_home) = env::var_os("XDG_DATA_HOME") {
      let mut path = PathBuf::from(xdg_config_home);
      path.push("fx");
      path.push("fxfile");
      paths.push(path);
    }

    Ok(paths)
  }
}
