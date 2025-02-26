use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};


#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about = "A dummy TOSCA processor intended as a stub for developing the validation test framework for TOSCA",
    long_about = "Accepts a path to a TOSCA file in an argument. Returns a pass or fail")]
struct Args {
    /// The path to the TOSCA file
    #[arg(short, long, help = "Path to the TOSCA file", default_value = "")]
    tosca_file: std::path::PathBuf,

}

fn main() -> Result<(), io::Error> {
    // Get the command-line arguments
    let args = Args::parse();

    if args.tosca_file.as_os_str().is_empty() {
        println!("No TOSCA file path provided. Use the -t argument to specify the file path.");
        return Ok(());
    }

    let file = File::open(&args.tosca_file).map_err(|error| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Error opening TOSCA file '{}': {}", &args.tosca_file.display(), error),
        )
    })?;
    


    let result = find_start(file);
    match result {
        Ok((start_found, deployable)) => {
            if start_found {
                if deployable {
                    println!("PASS");
                } else {
                    println!("TOSCA file is valid but not deployable");
                    std::process::exit(1);
                }
            } else {
                eprintln!("TOSCA start line not found");
                std::process::exit(1);
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
    Ok(())
    
}


fn find_start(file: File) -> Result<(bool, bool), io::Error> {
    
    let mut start_found = false;
    let mut deployable = true;
    let start_found_regex = Regex::new(r"tosca_definitions_version:").unwrap();
    let not_deployable_regex = Regex::new(r"not deployable").unwrap();

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if start_found_regex.is_match(line.trim()) {
                    // Found the start first line of TOSCA file
                    start_found = true;
                }

                if start_found {
                    if not_deployable_regex.is_match(line.trim()) {
                        deployable = false;
                        break;
                    }
                }
            }
            Err(error) => {
                return Err(io::Error::new(io::ErrorKind::Other, format!("Error reading line: {}", error)));
            }
        }
    }
    Ok((start_found, deployable))
}

