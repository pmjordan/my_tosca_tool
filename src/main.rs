//use assert_cmd::output;
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
    let reader = io::BufReader::new(file);


    let start_regex = Regex::new(r"^tosca_definitions_version: tosca_2.0").unwrap();

    for line in reader.lines() {
        let line = line?;

        if start_regex.is_match(line.trim()) {
            // Found the start first line of TOSCA file
            println!("PASS");
            return Ok(());
        }
        continue;
        
    }
    // Have not found the start of the TOSCA file
    Err(io::Error::new(io::ErrorKind::Other, "TOSCA start line not found"))
}
