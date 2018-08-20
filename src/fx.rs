use common::*;

pub struct Fx {
  patterns: Vec<Regex>,
}

impl Fx {
  pub fn from_file(path: PathBuf) -> Result<Fx, Vec<Error>> {
    let bytes = fs::read(&path).map_err(|io_error| Error::FxfileIo {
      io_error,
      path: path.clone(),
    })?;

    let text = String::from_utf8(bytes).map_err(|from_utf8_error| Error::FxfileNotUtf8 {
      from_utf8_error,
      path: path.clone(),
    })?;

    let mut patterns = Vec::new();

    let mut pattern_errors = Vec::new();

    for (line_index, line) in text.lines().enumerate() {
      if line.is_empty() || line.chars().next() == Some('#') {
        continue;
      }

      let pattern = format!("(?x){}", line);

      match Regex::new(&pattern) {
        Ok(pattern) => patterns.push(pattern),
        Err(regex_error) => pattern_errors.push(Error::FxfilePattern {
          regex_error,
          line_index,
          path: path.clone(),
        }),
      }
    }

    if !pattern_errors.is_empty() {
      return Err(pattern_errors);
    }

    if patterns.is_empty() {
      Err(Error::FxfileEmpty { path })?;
    }

    Ok(Fx { patterns })
  }

  pub fn matches<'text>(&self, text: &'text str) -> Vec<&'text str> {
    let all = self
      .patterns
      .iter()
      .flat_map(|pattern| {
        pattern
          .find_iter(text)
          .map(|match_object| match_object.as_str())
          .filter(|match_text| !match_text.is_empty())
      })
      .collect::<Vec<&str>>();

    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for matched_text in all {
      if !seen.contains(matched_text) {
        seen.insert(matched_text);
        unique.push(matched_text);
      }
    }

    unique
  }
}
