use crate::internal::manifest::{Dependency, Manifest};
use crate::internal::dirs;

use std::fs::{OpenOptions, create_dir_all};
use std::io::{stdout, Write, Result};
use std::process::Command;

pub fn deps_get(manifest: &Manifest) -> Result<()> {
  let ws = dirs::workspace()?;
  create_dir_all(&ws)?;

  println!("Fetching dependencies:");
  fetch_deps(&manifest.dependencies, Vec::new())?;

  Ok(())
}

fn fetch_deps(deps: &[Dependency], visited: Vec<String>) -> Result<()> {
  let ws = dirs::workspace()?;
  let logpath = ws.join("deps-get.log");
  let logfile = OpenOptions::new().append(true).create(true).open(&logpath)?;

  for dep in deps {
    print!("  - {}... ", dep.name);
    stdout().flush()?;

    if visited.contains(&dep.name) {
      println!("circular dependency detected");
      std::process::exit(1);
    }

    let build_dir = dirs::deps()?.join(&dep.name);

    let ret = if build_dir.exists() {
      Command::new("git")
        .arg("pull")
        .arg("--recurse-submodules")
        .current_dir(&build_dir)
        .stdout(logfile.try_clone()?)
        .stderr(logfile.try_clone()?)
        .status()?
    }
    else {
      Command::new("git")
        .arg("clone")
        .arg("--recurse-submodules")
        .arg(&dep.url)
        .arg(&build_dir)
        .stdout(logfile.try_clone()?)
        .stderr(logfile.try_clone()?)
        .status()?
    };

    if !ret.success() {
      println!("failed");
      eprintln!("ERROR: Failed to fetch dependency '{}'", dep.name);
      std::process::exit(1);
    }

    if let Some(version) = &dep.version {
      let ret = Command::new("git")
        .arg("checkout")
        .arg(version)
        .current_dir(&build_dir)
        .stdout(logfile.try_clone()?)
        .stderr(logfile.try_clone()?)
        .status()?;

      if !ret.success() {
        println!("failed");
        eprintln!("ERROR: Failed to checkout version '{}' for dependency '{}'", version, dep.name);
        std::process::exit(1);
      }
    }

    let local_manifest = build_dir.join("shipp.json");
    if local_manifest.exists() {
      println!("ok");

      let local_manifest = Manifest::from_file(&local_manifest)?;
      let mut visited = visited.clone();
      visited.push(dep.name.clone());
      fetch_deps(&local_manifest.dependencies, visited)?;
    }
    else {
      println!("failed");
      eprintln!("ERROR: No shipp.json found in dependency '{}'", dep.name);
      std::process::exit(1);
    }
  }

  Ok(())
}
