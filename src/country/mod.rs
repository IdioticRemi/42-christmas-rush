mod region;
use region::{Money, Region};

use rayon::prelude::*;
use rayon::vec::IntoIter;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Country {
    pub regions: HashMap<String, Region>,
}

impl Country {
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

    fn regions_to_fuse(&self, target_gdp: Money) -> (String, String) {
        let score = |(a, b): (&Region, &Region)| (a.gdp + b.gdp - target_gdp).abs();
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
        (best.0.name.clone(), best.1.name.clone())
    }

    fn regions_to_fuse_smallest(&self, target_gdp: Money) -> (String, String) {
        let score = |(a, b): (&Region, &Region)| (a.gdp + b.gdp - target_gdp).abs();
        let mut sorted: Vec<&Region> = self.regions.values().collect();
        sorted.sort_by(|a, b| a.gdp.partial_cmp(&b.gdp).unwrap_or(Ordering::Less));

        for region in sorted {
            let mut best: Option<(&Region, &Region)> = None;
            for other in region.links.iter().map(|r| &self.regions[r]) {
                match best {
                    None => best = Some((region, other)),
                    Some(ref mut best) => {
                        if score((region, other)) < score(*best) {
                            *best = (region, other);
                        }
                    }
                }
            }
            if let Some(best) = best {
                return (best.0.name.clone(), best.1.name.clone());
            }
        }
        panic!("Could not find any link")
    }

    pub fn organize(&mut self, count: usize) {
        let final_avg_gdp = self.total_gdp() / count as Money;

        while self.regions.len() > count {
            // let best = self.regions_to_fuse(final_avg_gdp);
            let best = self.regions_to_fuse_smallest(final_avg_gdp);
            self.fuse_regions((&best.0, &best.1));
        }
    }

    pub fn optimize(&mut self, target_count: usize) -> Result<(), ()> {
        match target_count.cmp(&self.regions.len()) {
            Ordering::Equal => return Ok(()),
            Ordering::Greater => return Err(()),
            _ => {}
        }
        if target_count >= self.regions.len() {
            return Err(());
        }
        let mut links: Vec<(String, String)> = self
            .regions
            .values()
            .map(|r| r.links.iter().map(|l| (r.name.clone(), l.clone())))
            .flatten()
            .collect();
        links.sort();
        links.dedup();
        let best = links
            .into_par_iter()
            .map(|link| {
                let mut cloned = self.clone();
                cloned.fuse_regions((link.0.as_ref(), link.1.as_ref()));
                // TODO change
                cloned.optimize(target_count).unwrap();
                cloned
            })
            .min_by(|a, b| {
                a.std_dev_sq()
                    .partial_cmp(&b.std_dev_sq())
                    .unwrap_or(Ordering::Less)
            });
        best.map(|b| *self = b).ok_or(())
    }

    fn remove_link_from_region(&mut self, region_name: &str, name: &str) {
        let pos = self.regions[region_name]
            .links
            .iter()
            .position(|r| r == &name);

        pos.map(|p| self.regions.get_mut(region_name).unwrap().links.remove(p));
    }

    fn add_link_to_region(&mut self, region_name: &str, name: &str) {
        self.regions
            .get_mut(region_name)
            .unwrap()
            .links
            .push(name.into());
    }

    fn fuse_regions(&mut self, (left_name, right_name): (&str, &str)) {
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

impl FromStr for Country {
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

impl fmt::Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "The avg GDP is {}", self.avg_gdp())?;
        writeln!(f, "The std_dev_sq is {}", self.std_dev_sq())?;
        writeln!(f, "The std_dev is {}", self.std_dev())
    }
}

#[cfg(test)]
mod tests {
    use crate::Country;

    const INPUT: &str = include_str!("../../subject/exempleRegion.txt");
    const INPUT_TEST: &str = include_str!("../../subject/exempleTest.txt");

    #[test]
    fn remove_link_from_region() {
        let mut country: Country = INPUT.parse().unwrap();

        country.remove_link_from_region("Nord", "Paris");
        country.remove_link_from_region("Nord", "Normandie");

        assert!(!country.regions["Nord"].links.contains(&"Paris".into()));
        assert!(!country.regions["Nord"].links.contains(&"Normandie".into()));
    }

    #[test]
    fn add_link_to_region() {
        let mut country: Country = INPUT.parse().unwrap();

        country.add_link_to_region("Nord", "Nouvelle-Acquitaine");

        assert!(country.regions["Nord"]
            .links
            .contains(&"Nouvelle-Acquitaine".into()));
    }

    #[test]
    fn region_fuse() {
        let mut country: Country = INPUT_TEST.parse().unwrap();

        country.fuse_regions(("A", "C"));

        assert_eq!(country.regions["A-C"].links, vec!["B", "D"]);
        assert_eq!(country.regions["B"].links, vec!["A-C"]);
        assert_eq!(country.regions["D"].links, vec!["A-C"]);
    }

    fn check_bidir_links(country: Country) {
        for region in country.regions.values() {
            for other in region.links.iter() {
                assert!(country.regions[other].links.contains(&region.name));
            }
        }
    }

    #[test]
    fn bidirectional_links() {
        check_bidir_links(INPUT.parse().unwrap());
        check_bidir_links(INPUT_TEST.parse().unwrap());
    }
}
