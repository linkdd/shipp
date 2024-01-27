use crate::internal::manifest::{Dependency, Manifest};
use crate::internal::dirs;

use std::fs::create_dir_all;
use std::process::Command;

pub fn deps_build(manifest: &Manifest) -> std::io::Result<()> {
  let ws = dirs::workspace()?;
  create_dir_all(&ws)?;

  build_and_install_deps(&manifest.dependencies)?;

  Ok(())
}

fn build_and_install_deps(deps: &[Dependency]) -> std::io::Result<()> {
  for dep in deps {
    let build_dir = dirs::deps()?.join(&dep.name);

    if !build_dir.exists() {
      eprintln!(
        "ERROR: Dependency '{}' not fetched, try running `deps.get` first",
        dep.name,
      );
      std::process::exit(1);
    }

    let local_manifest = build_dir.join("shipp.json");
    let local_manifest = Manifest::from_file(&local_manifest)?;
    build_and_install_deps(&local_manifest.dependencies)?;

    println!("===[ Building {} ]===", dep.name);

    let ret = Command::new("sh")
      .arg("-c")
      .arg(&local_manifest.scripts.build)
      .current_dir(&build_dir)
      .env("SHIPP_DIST_DIR", dirs::dist()?)
      .env("SHIPP_TARGET_ARCH", std::env::consts::ARCH)
      .env("SHIPP_TARGET_FAMILY", std::env::consts::FAMILY)
      .env("SHIPP_TARGET_OS", std::env::consts::OS)
      .status()?;

    if !ret.success() {
      eprintln!("ERROR: Failed to build dependency '{}'", dep.name);
      std::process::exit(1);
    }

    let ret = Command::new("sh")
      .arg("-c")
      .arg(&local_manifest.scripts.install)
      .current_dir(&build_dir)
      .env("SHIPP_DIST_DIR", dirs::dist()?)
      .env("SHIPP_TARGET_ARCH", std::env::consts::ARCH)
      .env("SHIPP_TARGET_FAMILY", std::env::consts::FAMILY)
      .env("SHIPP_TARGET_OS", std::env::consts::OS)
      .status()?;

    if !ret.success() {
      eprintln!("ERROR: Failed to install dependency '{}'", dep.name);
      std::process::exit(1);
    }
  }

  Ok(())
}
