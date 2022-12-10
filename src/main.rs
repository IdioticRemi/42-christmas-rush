use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

type Money = f64;

#[derive(Debug)]
struct Region {
    name: String,
    gdp: Money,
    links: Vec<String>,
}

impl Region {
    fn fuse(mut self, mut other: Self) -> Self {
        // dbg!(&self, &other);

        other
            .links
            .remove(other.links.iter().position(|r| r == &self.name).unwrap());
        self.links
            .remove(self.links.iter().position(|r| r == &other.name).unwrap());

        // dbg!(&self.links);

        self.links.extend(other.links);
        self.links.sort();
        self.links.dedup();

        // dbg!(&self.links);

        // println!("FUUUUUUUSIONN: {} et {}", self.name, other.name);

        Self {
            name: format!("{}-{}", self.name, other.name),
            gdp: self.gdp + other.gdp,
            links: self.links,
        }
    }
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
    fn total_gdp(&self) -> Money {
        self.regions.values().map(|r| r.gdp).sum()
    }

    fn avg_gdp(&self) -> Money {
        self.total_gdp() / self.regions.len() as Money
    }

    fn std_dev_sq(&self) -> Money {
        let avg_gdp = self.avg_gdp();

        self.regions
            .values()
            .map(|r| (r.gdp - avg_gdp).powi(2))
            .sum::<Money>()
            / self.regions.len() as Money
    }

    fn std_dev(&self) -> Money {
        self.std_dev_sq().sqrt()
    }

    fn organize(&mut self, count: usize) {
        let final_avg_gdp = self.total_gdp() / count as Money;
        let score = |(a, b): (&Region, &Region)| (a.gdp + b.gdp - final_avg_gdp).abs();

        while self.regions.len() > count {
            // println!("-----------------------------------------------------");

            let mut it = self.regions.values();
            let mut best: Option<(&Region, &Region)> = None;

            for region in self.regions.values() {
                for other in region.links.iter().map(|o| &self.regions[o]) {
                    match best {
                        None => best = Some((region, other)),
                        Some(ref mut best) => {
                            if score((region, other)) < score(*best) {
                                *best = (region, other);
                            }
                        }
                    }
                }
            }

            let best = best.unwrap();

            // println!("{}", score(best));

            self.fuse_regions((&best.0.name.clone(), &best.1.name.clone()));
        }
    }

    fn remove_link_from_region(&mut self, region_name: &str, name: &str) {
        let pos = self.regions[region_name]
            .links
            .iter()
            .position(|r| r == &name);

        pos.map(|p| self.regions.get_mut(region_name).unwrap().links.remove(p));

        // println!("Removed link {} from {}", name, region_name);
    }

    fn add_link_to_region(&mut self, region_name: &str, name: &str) {
        self.regions
            .get_mut(region_name)
            .unwrap()
            .links
            .push(name.into());

        // println!("Added link {} to {}", name, region_name);
    }

    fn fuse_regions(&mut self, (left_name, right_name): (&str, &str)) {
        // println!("{} and {} are fusing...", left_name, right_name);

        let left = self.regions.remove(left_name).unwrap();
        let right = self.regions.remove(right_name).unwrap();
        let fused = left.fuse(right);

        for region_name in &fused.links {
            self.remove_link_from_region(region_name, left_name);
            self.remove_link_from_region(region_name, right_name);

            self.add_link_to_region(region_name, &fused.name);
        }

        self.regions.insert(fused.name.clone(), fused);
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

impl fmt::Display for France {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "The avg GDP is {}", self.avg_gdp())?;
        writeln!(f, "The std_dev_sq is {}", self.std_dev_sq())?;
        writeln!(f, "The std_dev is {}", self.std_dev())
    }
}

fn main() {
    let mut args = std::env::args().skip(1);

    let inputFile = args.next().unwrap();
    let outputFile = args.next().unwrap();
    let targetRegionCount: usize = args.next().unwrap().parse().unwrap();

    let input = std::fs::read_to_string(inputFile).unwrap();

    let mut france: France = input.parse().unwrap();

    // println!("{france}");

    france.organize(targetRegionCount);

    println!("{france}");
    // dbg!(&france);

    let mut regions: Vec<_> = france.regions.values().map(|r| r.name.as_ref()).collect();
    regions.sort();

    std::fs::write(outputFile, regions.join("\n")).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::France;

    const INPUT: &str = include_str!("../subject/exempleRegion.txt");
    const INPUT_TEST: &str = include_str!("../subject/exempleTest.txt");
    const INPUT_REAL: &str = include_str!("../subject/exempleRegionsReal.txt");

    #[test]
    fn remove_link_from_region() {
        let mut france: France = INPUT.parse().unwrap();

        france.remove_link_from_region("Nord", "Paris");
        france.remove_link_from_region("Nord", "Normandie");

        assert!(!france.regions["Nord"].links.contains(&"Paris".into()));
        assert!(!france.regions["Nord"].links.contains(&"Normandie".into()));
    }

    #[test]
    fn add_link_to_region() {
        let mut france: France = INPUT.parse().unwrap();

        france.add_link_to_region("Nord", "Nouvelle-Acquitaine");

        assert!(france.regions["Nord"]
            .links
            .contains(&"Nouvelle-Acquitaine".into()));
    }

    #[test]
    fn region_fuse() {
        let mut france: France = INPUT_TEST.parse().unwrap();

        france.fuse_regions(("A", "C"));

        assert_eq!(france.regions["A-C"].links, vec!["B", "D"]);
        assert_eq!(france.regions["B"].links, vec!["A-C"]);
        assert_eq!(france.regions["D"].links, vec!["A-C"]);
    }

    fn check_bidir_links(france: France) {
        let mut missing_links = vec![];

        for region in france.regions.values() {
            for other in region.links.iter() {
                if !france.regions[other].links.contains(&region.name) {
                    missing_links.push(format!("{} is not linked with {}", other, region.name));
                }
                // assert!(france.regions[other].links.contains(&region.name), "", other, region.name);
            }
        }

        assert!(missing_links.is_empty(), "\n{}\n", missing_links.join("\n"))
    }

    #[test]
    fn bidirectional_links() {
        check_bidir_links(INPUT.parse().unwrap());
        check_bidir_links(INPUT_TEST.parse().unwrap());
        check_bidir_links(INPUT_REAL.parse().unwrap());
    }
}
