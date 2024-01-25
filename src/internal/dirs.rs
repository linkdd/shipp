use std::path::PathBuf;

pub fn toplevel() -> std::io::Result<PathBuf> {
  match std::env::var("SHIPP_TOPLEVEL_DIR") {
    Ok(dir) => Ok(PathBuf::from(dir)),
    Err(_) => {
      let mut cwd = std::env::current_dir()?;

      loop {
        if (cwd.join("shipp.json")).exists() {
          break Ok(cwd);
        }

        if !cwd.pop() {
          break Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find shipp.json in any parent directory",
          ));
        }
      }
    },
  }
}

pub fn workspace() -> std::io::Result<PathBuf> {
  toplevel().map(|d| d.join(".shipp"))
}

pub fn deps() -> std::io::Result<PathBuf> {
  workspace().map(|d| d.join("deps"))
}

pub fn dist() -> std::io::Result<PathBuf> {
  workspace().map(|d| d.join("dist"))
}
