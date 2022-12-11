#![allow(dead_code)]

//! This module holds the deprected organise methods
use super::{Country, Money, Region};
use std::cmp::Ordering;

pub type Strategy = fn(&Country, gdp: Money) -> (String, String);

impl Country {
    pub fn organize(&mut self, count: usize, strategy: Strategy) {
        let final_avg_gdp = self.total_gdp() / count as Money;

        while self.regions.len() > count {
            let best = strategy(self, final_avg_gdp);
            self.fuse_regions((&best.0, &best.1));
        }
    }
}

// Strategies

/// Find the fusion that results in the GDP that's the clossest to the <target_gdp>
pub fn find_fusion_clossest_std_dev(country: &Country, target_gdp: Money) -> (String, String) {
    let score = |(a, b): (&Region, &Region)| (a.gdp + b.gdp - target_gdp).abs();
    let mut best: Option<(&Region, &Region)> = None;
    for region in country.regions.values() {
        for other in region.links.iter().map(|o| &country.regions[o]) {
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
    let best = best.expect("Could not find any link to fuze");
    (best.0.name.clone(), best.1.name.clone())
}

/// Finds the fusions using the smallest region
/// resulting in a GDP that the closest to <target_gdp>
pub fn find_fusion_clossest_std_dev_with_priority_to_smallest(
    country: &Country,
    target_gdp: Money,
) -> (String, String) {
    let score = |(a, b): (&Region, &Region)| (a.gdp + b.gdp - target_gdp).abs();
    let mut sorted: Vec<&Region> = country.regions.values().collect();
    sorted.sort_by(|a, b| a.gdp.partial_cmp(&b.gdp).unwrap_or(Ordering::Less));

    for region in sorted {
        let mut best: Option<(&Region, &Region)> = None;
        for other in region.links.iter().map(|r| &country.regions[r]) {
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
