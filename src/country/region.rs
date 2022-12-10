use std::str::FromStr;

pub type Money = f64;

#[derive(Debug)]
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
