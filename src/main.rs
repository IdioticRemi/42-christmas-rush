mod country;
use country::Country;

fn parse_args() -> Result<(String, String, usize), ()> {
    let mut args = std::env::args().skip(1);
    let input_file = args.next().ok_or(())?;
    let output_file = args.next().ok_or(())?;
    let target_region_count: usize = args.next().ok_or(())?.parse().map_err(|_| ())?;
    Ok((input_file, output_file, target_region_count))
}

fn show_help() -> ! {
    println!("./rush-nowel <input file> <output file> <region count>");
    std::process::exit(1)
}

fn main() {
    let (input_file, output_file, target_region_count) = match parse_args() {
        Ok(t) => t,
        Err(_) => show_help(),
    };

    let error_output_file = output_file.clone();
    std::panic::set_hook(Box::new(move |_| {
        if let Err(error) = std::fs::write(&error_output_file,"Error\n") {
            eprintln!("Could not write to {error_output_file}: {error}");
        }
        std::process::exit(1);
    }));

    let input = std::fs::read_to_string(input_file).unwrap();

    let mut country: Country = input.parse().unwrap();

    assert_ne!(target_region_count, 0);
    assert!(target_region_count > country.regions.len());

    country.optimize3(target_region_count).unwrap();

    println!("{country}");

    let mut regions: Vec<_> = country.regions.values().map(|r| r.name.as_ref()).collect();
    regions.sort();

    std::fs::write(output_file, regions.join("\n")).unwrap();
}
