use crate::internal::manifest::Manifest;
use crate::internal::dirs;

use std::fs::create_dir_all;
use std::process::Command;

pub fn build(manifest: &Manifest) -> std::io::Result<()> {
  let ws = dirs::workspace()?;
  create_dir_all(&ws)?;

  println!("===[ Building project ]===");

  for dep in &manifest.dependencies {
    let dep_dir = dirs::deps()?.join(&dep.name);
    if !dep_dir.exists() {
      eprintln!(
        "ERROR: Dependency '{}' not fetched, try running `deps.get` and `deps.build` first",
        dep.name,
      );
      std::process::exit(1);
    }
  }

  let build_dir = dirs::toplevel()?;

  let ret = Command::new("sh")
    .arg("-c")
    .arg(&manifest.scripts.build)
    .current_dir(&build_dir)
    .env("SHIPP_DIST_DIR", dirs::dist()?)
    .env("SHIPP_TARGET_ARCH", std::env::consts::ARCH)
    .env("SHIPP_TARGET_FAMILY", std::env::consts::FAMILY)
    .env("SHIPP_TARGET_OS", std::env::consts::OS)
    .status()?;

  if !ret.success() {
    eprintln!("ERROR: Failed to build project");
    std::process::exit(1);
  }

  let ret = Command::new("sh")
    .arg("-c")
    .arg(&manifest.scripts.install)
    .current_dir(&build_dir)
    .env("SHIPP_DIST_DIR", dirs::dist()?)
    .env("SHIPP_TARGET_ARCH", std::env::consts::ARCH)
    .env("SHIPP_TARGET_FAMILY", std::env::consts::FAMILY)
    .env("SHIPP_TARGET_OS", std::env::consts::OS)
    .status()?;

  if !ret.success() {
    eprintln!("ERROR: Failed to install project");
    std::process::exit(1);
  }

  Ok(())
}
