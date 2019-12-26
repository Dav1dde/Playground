extern crate itertools;

use std::fs;
use std::collections::HashMap;

use itertools::Itertools;


#[derive(Debug, Clone)]
struct Chemical {
    quantity: u32,
    name: String
}

impl Chemical {
    fn parse(chemical: &str) -> Self {
        let (quantity, name) = chemical.trim().split_whitespace()
            .next_tuple().expect("invalid chemical");
        Chemical { quantity: quantity.parse().unwrap(), name: name.to_string() }
    }

    fn scale(&self, scale: u32) -> Chemical {
        Chemical { quantity: self.quantity * scale, name: self.name.clone() }
    }

    fn with_quantity(&self, quantity: u32) -> Chemical {
        Chemical { quantity: quantity, name: self.name.clone() }
    }
}


#[derive(Debug, Clone)]
struct Reaction {
    requires: Vec<Chemical>,
    produces: Chemical
}

impl Reaction {
    fn parse(input: &str) -> Self {
        let (left, right) = input.trim().splitn(2, "=>")
            .next_tuple().expect("invalid reaction");
        let requires = left.split(",").map(|c| Chemical::parse(c)).collect();
        Reaction { requires: requires, produces: Chemical::parse(right) }
    }

    fn produces(&self, name: &str, quantity: u32) -> Option<(Vec<Chemical>, Chemical)> {
        if self.produces.name.as_str() != name {
            return None;
        }

        let scale = (quantity as f32 / self.produces.quantity as f32).ceil() as u32;

        let requires = self.requires.iter()
            .map(|chemical| chemical.scale(scale))
            .collect();
        let leftover = self.produces.with_quantity((scale * self.produces.quantity) - quantity);

        Some((requires, leftover))
    }
}


#[derive(Debug)]
struct System {
    reactions: Vec<Reaction>
}

impl System {
    fn parse(input: &str) -> Self {
        let reactions = input.lines().map(|reaction| Reaction::parse(reaction)).collect();
        System { reactions: reactions }
    }

    fn produce(&self, name: &str, quantity: u32, store: &mut HashMap<String, u32>) -> Vec<Chemical> {
        let mut total = Vec::new();

        for reaction in self.reactions.iter() {
            let result = match reaction.produces(name, quantity) {
                None => continue,
                result => result.unwrap()
            };

            *store.entry(result.1.name.clone()).or_insert(0) += result.1.quantity;

            for chemical in result.0 {
                let stored = *store.get(&chemical.name).unwrap_or(&0);

                // TODO make this nicer
                let mut quant = 0;
                if stored >= chemical.quantity {
                    store.insert(chemical.name.clone(), stored - chemical.quantity);
                    quant = 0;
                } else if stored < chemical.quantity {
                    store.insert(chemical.name.clone(), 0);
                    quant = chemical.quantity - stored;
                }

                let mut other = self.produce(chemical.name.as_str(), quant, store);
                if other.len() > 0 {
                    total.append(&mut other);
                } else {
                    total.push(chemical);
                }

            }
        }

        total
    }
}


fn main() {
    let s1 = System::parse(fs::read_to_string("../a.txt").unwrap().as_str());
    let s2 = System::parse(fs::read_to_string("../b.txt").unwrap().as_str());
    let s3 = System::parse(fs::read_to_string("../input.txt").unwrap().as_str());

    let system = s3;

    // println!("{:?}", system);
    let x = system.produce("FUEL", 1, &mut HashMap::new()).iter()
        .map(|c| (c.name.clone(), c.quantity))
        .into_group_map()
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().sum::<u32>()))
        .into_group_map();

    println!("{:?}", x);
}
