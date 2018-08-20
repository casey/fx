use common::*;

pub enum Location<'path> {
  Path {
    path: &'path Path,
  },
  Line {
    path: &'path Path,
    line_index: usize,
  },
}

impl<'path> Display for Location<'path> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Location::Path { path } => write!(f, "  --> {}", path.display()),
      Location::Line { path, line_index } => {
        write!(f, "  --> {}:{}", path.display(), line_index + 1)
      }
    }
  }
}
