use std::{str::FromStr, error::Error};

pub type Money = f64;

#[derive(Debug, Clone)]
pub struct Region {
    pub name: String,
    pub gdp: Money,
    pub links: Vec<String>,
}

impl Region {
    pub fn fuse(mut self, mut other: Self) -> Self {
        other
            .links
            .remove(other.links.iter().position(|r| r == &self.name).unwrap());
        self.links
            .remove(self.links.iter().position(|r| r == &other.name).unwrap());

        self.links.extend(other.links);
        self.links.sort();
        self.links.dedup();

        Self {
            name: format!("{}-{}", self.name, other.name),
            gdp: self.gdp + other.gdp,
            links: self.links,
        }
    }
}

impl FromStr for Region {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line_args = line.split(" : ");

        let name = line_args.next()
            .ok_or("Missing region name")?
            .to_owned();
        let gdp = line_args.next()
            .ok_or("Missing region gdp")?
            .parse()
            .map_err(|e| format!("Could not parse gdp: {}", e))?;
        let links = line_args
            .next()
            .ok_or("Missing region links")?
            .split("-")
            .map(ToOwned::to_owned)
            .collect();

        Ok(Region { name, gdp, links })
    }
}
