use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::fmt;
use std::fs;


type PlanetId = String;


struct Planet {
    id: PlanetId,
    orbits: Option<Rc<RefCell<Planet>>>,
    orbited_by: Vec<Weak<RefCell<Planet>>>
}

impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent = self.orbits.clone().map(|x| x.borrow().id.clone());
        write!(
            f,
            "Planet {{ id: {}, orbits: {:?}, orbited_by: {:?} }}",
            self.id, parent, self.orbited_by
        )
    }
}

impl Planet {
    fn new(id: PlanetId) -> Self {
        Planet { id: id, orbits: None, orbited_by: Vec::new() }
    }

    fn orbits(&mut self, parent: Rc<RefCell<Planet>>) {
        self.orbits = Some(parent);
    }

    fn orbited_by(&mut self, other: Weak<RefCell<Planet>>) {
        self.orbited_by.push(other);
    }

    fn is_root(&self) -> bool {
        self.orbits.is_none()
    }

    fn depth(&self) -> usize {
        match self.orbits {
            Some(ref parent) => 1 + parent.borrow().depth(),
            None => 0
        }
    }

    fn indirect_orbits(&self) -> usize {
        match self.depth() {
            0 => 0,
            1 => 0,
            x => x - 1
        }
    }

    fn all_orbits(&self) -> Vec<PlanetId> {
        if self.orbits.is_none() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let a = self.orbits.clone().unwrap();
        result.push(a.borrow().id.clone());
        result.extend(a.borrow().all_orbits());
        result
    }
}


#[derive(Debug)]
struct Atlas {
    planets: HashMap<PlanetId, Rc<RefCell<Planet>>>
}

impl Atlas {
    fn new() -> Self {
        Atlas { planets: HashMap::new() }
    }

    fn create_orbiting_planet(&mut self, id: &PlanetId, orbiting: &PlanetId) {
        let planet = self.planets.entry(id.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Planet::new(id.clone()))))
            .clone();

        let orbiting = self.planets.entry(orbiting.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Planet::new(orbiting.clone()))))
            .clone();

        orbiting.borrow_mut().orbited_by(Rc::downgrade(&planet));
        planet.borrow_mut().orbits(orbiting);
    }

    fn roots(&self) -> Vec<PlanetId> {
        self.planets.values()
            .filter(|planet| planet.borrow().is_root())
            .map(|planet| planet.borrow().id.clone())
            .collect()
    }

    fn direct_orbits(&self) -> usize {
        self.planets.values()
            .filter(|planet| !planet.borrow().is_root())
            .count()
    }

    fn indirect_orbits(&self) -> usize {
        self.planets.values()
            .map(|planet| planet.borrow().indirect_orbits())
            .sum()
    }

    fn total_orbits(&self) -> usize {
        self.direct_orbits() + self.indirect_orbits()
    }

    fn num_planets(&self) -> usize {
        self.planets.len()
    }

    fn orbital_transfer(&self, from: &PlanetId, to: &PlanetId) -> Option<usize> {
        let from_orbits = self.planets.get(from).unwrap().borrow().all_orbits();
        let to_orbits = self.planets.get(to).unwrap().borrow().all_orbits();

        for (index, planet) in from_orbits.iter().enumerate() {
            if let Some(position) = to_orbits.iter().position(|x| x == planet) {
                return Some(index + position)
            }
        }

        None
    }
}


fn main() {
    let mut atlas = Atlas::new();

    let input = fs::read_to_string("../input.txt").unwrap();

    for line in input.lines() {
        let mut s = line.split(")");
        let parent = s.next().unwrap();
        let planet = s.next().unwrap();
        atlas.create_orbiting_planet(&planet.to_string(), &parent.to_string());
    }

    println!("root: {:?}", atlas.roots());
    println!("planets: {:?}", atlas.num_planets());
    println!("direct orbits: {:?}", atlas.direct_orbits());
    println!("indirect orbits: {:?}", atlas.indirect_orbits());
    println!("total orbits: {:?}", atlas.total_orbits());
    println!(
        "transfer from YOU to SAN: {:?}",
        atlas.orbital_transfer(&"YOU".to_string(), &"SAN".to_string())
    );
}

