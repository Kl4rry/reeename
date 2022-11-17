use std::{fs, path::PathBuf, process::ExitCode};

use anyhow::Result;
use clap::Parser;
use rustyline::error::ReadlineError;

/// Rename files easily
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to rename
    path: PathBuf,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();
    let Some(name) = args.path.file_name() else {
        eprintln!("'{}' has no name", args.path.to_string_lossy());
        return Ok(ExitCode::from(1));
    };

    if !args.path.exists() {
        eprintln!("Error '{}' does not exist", args.path.to_string_lossy());
        return Ok(ExitCode::from(1));
    }

    let name = name.to_string_lossy().to_string();
    let mut rl = rustyline::Editor::<()>::new()?;
    let new_name = match rl.readline_with_initial("New name: ", (&name, "")) {
        Ok(line) => line,
        Err(ReadlineError::Interrupted) => {
            eprintln!("^C");
            return Ok(ExitCode::from(1));
        }
        Err(ReadlineError::Eof) => {
            eprintln!("^D");
            return Ok(ExitCode::from(1));
        }
        Err(err) => Err(err)?,
    };

    let mut new_path = args.path.clone();
    new_path.pop();
    new_path.push(new_name);

    fs::rename(args.path, new_path)?;
    Ok(ExitCode::from(0))
}
