use crate::internal::manifest::{Dependency, Manifest};
use crate::internal::dirs;

use std::collections::HashMap;
use std::fs::create_dir_all;
use std::process::Command;

pub fn deps_get(manifest: &Manifest) -> std::io::Result<()> {
  let ws = dirs::workspace()?;
  create_dir_all(&ws)?;

  let mut cache = HashMap::new();
  let visited = Vec::new();
  fetch_deps(&manifest.dependencies, visited, &mut cache)?;

  Ok(())
}

fn fetch_deps(
  deps: &[Dependency],
  visited: Vec<String>,
  cache: &mut HashMap<String, bool>,
) -> std::io::Result<()> {
  let deps: Vec<&Dependency> = {
    deps
      .iter()
      .filter(|dep| !cache.get(&dep.name).unwrap_or(&false))
      .collect()
  };

  for dep in deps {
    println!("===[ Fetch {} ]===", dep.name);

    if visited.contains(&dep.name) {
      eprintln!("ERROR: Circular dependency detected");
      std::process::exit(1);
    }

    let build_dir = dirs::deps()?.join(&dep.name);

    let ret = if build_dir.exists() {
      Command::new("git")
        .arg("pull")
        .arg("--recurse-submodules")
        .current_dir(&build_dir)
        .status()?
    }
    else {
      Command::new("git")
        .arg("clone")
        .arg("--recurse-submodules")
        .arg(&dep.url)
        .arg(&build_dir)
        .status()?
    };

    if !ret.success() {
      eprintln!("ERROR: Failed to fetch dependency '{}'", dep.name);
      std::process::exit(1);
    }

    if let Some(version) = &dep.version {
      let ret = Command::new("git")
        .arg("checkout")
        .arg(version)
        .current_dir(&build_dir)
        .status()?;

      if !ret.success() {
        eprintln!("ERROR: Failed to checkout version '{}' for dependency '{}'", version, dep.name);
        std::process::exit(1);
      }
    }

    cache.insert(dep.name.clone(), true);

    let local_manifest = build_dir.join("shipp.json");
    if local_manifest.exists() {
      let local_manifest = Manifest::from_file(&local_manifest)?;
      let mut visited = visited.clone();
      visited.push(dep.name.clone());
      fetch_deps(&local_manifest.dependencies, visited, cache)?;
    }
    else {
      eprintln!("ERROR: No shipp.json found in dependency '{}'", dep.name);
      std::process::exit(1);
    }
  }

  Ok(())
}
