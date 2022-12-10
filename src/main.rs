mod country;
use country::Country;

fn main() {
    let mut args = std::env::args().skip(1);

    let input_file = args.next().unwrap();
    let output_file = args.next().unwrap();
    let target_region_count: usize = args.next().unwrap().parse().unwrap();

    let input = std::fs::read_to_string(input_file).unwrap();

    let mut country: Country = input.parse().unwrap();
    println!("{country}");

    // country.organize(target_region_count);
    country.optimize(target_region_count).unwrap();

    println!("{country}");

    let mut regions: Vec<_> = country.regions.values().map(|r| r.name.as_ref()).collect();
    regions.sort();

    std::fs::write(output_file, regions.join("\n")).unwrap();
}
