use std::path::Path;
use serde::Deserialize;
use super::dirs;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Manifest {
  pub name: String,
  pub version: Option<String>,
  pub scripts: Scripts,
  #[serde(default)]
  pub dependencies: Vec<Dependency>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Scripts {
  pub build: String,
  pub install: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Dependency {
  pub name: String,
  pub url: String,
  pub version: Option<String>,
}

impl Manifest {
  pub fn toplevel() -> std::io::Result<Self> {
    let path = dirs::toplevel()?.join("shipp.json");
    Self::from_file(&path)
  }

  pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
    let file = std::fs::File::open(path)?;
    let manifest = serde_json::from_reader(file)?;
    Ok(manifest)
  }
}
