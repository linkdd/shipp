use shipp::internal::manifest::Manifest;
use shipp::cmd;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[command(subcommand)]
  cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  #[command(name = "deps.get", about = "Fetch project dependencies")]
  DepsGet,
  #[command(name = "deps.build", about = "Build project dependencies")]
  DepsBuild,
  #[command(name = "build", about = "Build project")]
  Build,
  #[command(name = "dist", about = "Create distribution package")]
  Dist,
}

impl Command {
  fn run(&self, manifest: &Manifest) -> std::io::Result<()> {
    match self {
      Command::DepsGet => cmd::deps_get(manifest),
      Command::DepsBuild => cmd::deps_build(manifest),
      Command::Build => cmd::build(manifest),
      Command::Dist => cmd::dist(manifest),
    }
  }
}

fn main() {
  let cli = Cli::parse();

  match Manifest::toplevel().and_then(|m| cli.cmd.run(&m)) {
    Ok(_) => (),
    Err(err) => {
      eprintln!("ERROR: {}", err);
      std::process::exit(1);
    }
  }
}
