use common::*;

pub fn capture() -> Result<String, Error> {
  let command = &[
    "tmux",
    "capture-pane",
    "-p", // output to stdout
    "-J", // join lines
  ];

  let process::Output {
    status,
    stdout,
    stderr,
  } = Command::new(command[0])
    .args(&command[1..])
    .output()
    .map_err(|io_error| Error::CaptureExecution { io_error, command })?;

  let stderr = String::from_utf8_lossy(&stderr).to_string();

  let mut stdout = String::from_utf8_lossy(&stdout).to_string();

  if !status.success() {
    return Err(Error::CaptureStatus {
      status,
      stderr,
      stdout,
      command,
    });
  }

  if !stderr.is_empty() {
    eprintln!("{}", stderr);
  }

  stdout.retain(|c| c != '\0');

  Ok(stdout)
}
