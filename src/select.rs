use common::*;

pub fn select<'text>(matches: &[&'text str]) -> Result<Vec<&'text str>, Error> {
  let command = &[
    "fzf",
    "-m",            // allow multiple selections,
    "--border",      // draw a border around finder
    "--inline-info", // display finder info inline with query
    "--header=FX",   // display header
    "--ansi",        // use color codes
    "--tabstop=4",   // four space tabs
    "--read0",       // expect null-byte delimited input
    "--print0",      // write null-byte delimited output
  ];

  let mut fzf = Command::new(command[0])
    .args(&command[1..])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .unwrap(); // FIX

  for match_text in matches {
    let stdin = fzf.stdin.as_mut().unwrap(); // FIX
    stdin.write_all(match_text.as_bytes()).unwrap(); // FIX
    stdin.write_all("\0".as_bytes()).unwrap(); // FIX
  }

  fzf.wait().unwrap(); // FIX

  let mut output = String::new();

  fzf
    .stdout
    .take()
    .unwrap() // FIX
    .read_to_string(&mut output)
    .unwrap(); // FIX

  let match_set = matches.iter().cloned().collect::<HashSet<&str>>();

  let mut results = Vec::new();

  let mut unmatched = Vec::new();

  println!("output: {:?}", output);

  for result in output.split('\0') {
    if result.is_empty() {
      continue;
    }

    if let Some(&matched) = match_set.get(result) {
      results.push(matched);
    } else {
      unmatched.push(result.to_string());
    }
  }

  if !unmatched.is_empty() {
    return Err(Error::UnmatchedSelections {
      unmatched,
      matches: matches.iter().cloned().map(str::to_string).collect(),
    });
  }

  Ok(results)
}
