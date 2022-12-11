mod country;
use std::{
    env::{args, Args},
    error::Error,
    fs,
};

use country::Country;

struct Config {
    input_file: String,
    output_file: String,
    target_region_count: usize,
}

impl Config {
    fn from_args(mut args: Args) -> Result<Self, Box<dyn Error>> {
        let _ = args.next(); // Skip program name
        let input_file = args.next().ok_or("Missing <input file>")?;
        let output_file = args.next().ok_or("Missing <output file>")?;
        let target_region_count: usize = args.next().ok_or("Missing <region count>")?.parse()?;
        Ok(Self {
            input_file,
            output_file,
            target_region_count,
        })
    }
}

fn exit_with_help() -> ! {
    let command_name = args().next().unwrap_or("<unknown>".into());
    eprintln!("How to run:");
    eprintln!("\t{command_name} <input file> <output file> <region count>");
    std::process::exit(1)
}

fn do_work(config: &Config) -> Result<(), Box<dyn Error>> {
    // Read input file
    let input = fs::read_to_string(&config.input_file)?;
    // Parse country region info
    let mut country: Country = input.parse()?;

    // Optmize disposition with algorithm 3
    country.optimize3(config.target_region_count)?;

    // Collect all resulting region names
    let mut regions: Vec<_> = country.regions.values().map(|r| r.name.as_ref()).collect();
    // Sort for more consistency
    regions.sort();

    // Write down final result
    fs::write(&config.output_file, regions.join("\n"))?;

    // Everything went fine
    Ok(())
}

fn main() {
    // Parse arguments
    let config = match Config::from_args(args()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error:\n\t{error}");
            exit_with_help()
        }
    };

    // Read process & write to output
    match do_work(&config) {
        Ok(()) => println!("Done!"),
        Err(error) => {
            eprintln!("Error: {error}");
            let io_result = fs::write(&config.output_file, "Error\n");
            if let Err(io_error) = io_result {
                eprintln!(
                    "\t Could not write error to {}: {}",
                    config.output_file, io_error
                )
            }
        }
    }
}
