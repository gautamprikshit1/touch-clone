use std::{
    error::Error,
    fs::File,
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use filetime::{set_file_mtime, FileTime};

#[derive(Debug, Parser)]
pub struct Args {
    name: PathBuf,
}

fn touch(file: &Path) -> Result<(), Box<dyn Error>> {
    if file.exists() {
        set_file_mtime(file, FileTime::now())?;
    } else {
        File::create(file)?;
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(ref e) = touch(&args.name) {
        println!("{}", e);
        process::exit(1);
    }
    println!("Modified {}", args.name.to_str().unwrap());
}
