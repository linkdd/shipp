use crate::internal::manifest::Manifest;
use crate::internal::dirs;

use std::fs::OpenOptions;
use std::io::{stdout, Result, Write};

use flate2::write::GzEncoder;
use flate2::Compression;

pub fn dist(manifest: &Manifest) -> Result<()> {
  print!("Packaging project... ");
  stdout().flush()?;

  let (name, version) = (
    manifest.name.clone(),
    manifest.version.clone().unwrap_or(String::from("latest"))
  );

  let pkg_path = dirs::toplevel()?.join(
    format!("{}-{}.tar.gz", name, version)
  );

  let pkg_archive = OpenOptions::new().write(true).create(true).open(&pkg_path)?;
  let enc = GzEncoder::new(pkg_archive, Compression::default());
  let mut tar = tar::Builder::new(enc);
  tar.append_dir_all(
    format!("{}-{}", name, version),
    dirs::dist()?
  )?;

  Ok(())
}
