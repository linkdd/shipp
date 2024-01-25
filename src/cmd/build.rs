use crate::internal::manifest::Manifest;
use crate::internal::dirs;

use std::fs::{OpenOptions, create_dir_all};
use std::io::{stdout, Result, Write};
use std::process::Command;

pub fn build(manifest: &Manifest) -> Result<()> {
  let ws = dirs::workspace()?;
  create_dir_all(&ws)?;

  let logpath = ws.join("build.log");
  let logfile = OpenOptions::new().append(true).create(true).open(&logpath)?;

  print!("Building project... ");
  stdout().flush()?;

  for dep in &manifest.dependencies {
    let dep_dir = dirs::deps()?.join(&dep.name);
    if !dep_dir.exists() {
      println!("failed");
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
    .stdout(logfile.try_clone()?)
    .stderr(logfile.try_clone()?)
    .status()?;

  if !ret.success() {
    println!("failed");
    eprintln!("ERROR: Failed to build project");
    std::process::exit(1);
  }

  let ret = Command::new("sh")
    .arg("-c")
    .arg(&manifest.scripts.install)
    .current_dir(&build_dir)
    .env("SHIPP_DIST_DIR", dirs::dist()?)
    .stdout(logfile.try_clone()?)
    .stderr(logfile.try_clone()?)
    .status()?;

  if !ret.success() {
    println!("failed");
    eprintln!("ERROR: Failed to install project");
    std::process::exit(1);
  }

  println!("ok");

  Ok(())
}
