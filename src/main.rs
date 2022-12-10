use std::collections::HashMap;
use std::str::FromStr;

type Money = f64;

#[derive(Debug)]
struct Region {
    name: String,
    gdp: Money,
    links: Vec<String>,
}

impl FromStr for Region {
    // TODO: Faire les erreurs
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line_args = line.split(" : ");

        let name = line_args.next().unwrap().to_owned();
        let gdp = line_args.next().unwrap().parse().unwrap();
        let links = line_args
            .next()
            .unwrap()
            .split("-")
            .map(ToOwned::to_owned)
            .collect();

        Ok(Region { name, gdp, links })
    }
}

#[derive(Debug)]
struct France {
    regions: HashMap<String, Region>,
}

impl France {
    fn mean_gdp(&self) -> Money {
        let sum: Money = self.regions.values().map(|r| r.gdp).sum();

        sum / self.regions.len() as Money
    }

    fn std_dev_sq(&self) -> Money {
        let mean_gdp = self.mean_gdp();

        self.regions
            .values()
            .map(|r| (r.gdp - mean_gdp).powi(2))
            .sum::<Money>()
            / self.regions.len() as Money
    }

    fn std_dev(&self) -> Money {
        self.std_dev_sq().sqrt()
    }
}

impl FromStr for France {
    // TODO: Faire les erreurs
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let regions = input
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|r: Region| (r.name.clone(), r))
            .collect();

        Ok(Self { regions })
    }
}

fn main() {
    let input = include_str!("../sujet/exempleRegion.txt");
    let france: France = input.parse().unwrap();

    println!("The mean GDP is {}", france.mean_gdp());
    println!("The std_dev_sq is {}", france.std_dev_sq());
    println!("The std_dev is {}", france.std_dev());
}
