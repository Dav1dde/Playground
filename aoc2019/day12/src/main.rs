extern crate itertools;
extern crate num;
use itertools::Itertools;
use num::Integer;
use std::fmt;
use std::cell::RefCell;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl<'a> std::ops::Sub for &'a Vector {
    type Output = Vector;
    fn sub(self, other: &'a Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl<'a> std::ops::Add for &'a Vector {
    type Output = Vector;
    fn add(self, other: &'a Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x: {:3}, y: {:3}, z: {:3}>", self.x, self.y, self.z)
    }
}

impl Vector {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Vector { x, y, z }
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Moon {
    name: String,
    position: Vector,
    velocity: Vector
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:10}: pos={} vel={} pot=<{:3}> kin=<{:3}> total=<{:3}>",
            self.name, self.position, self.velocity,
            self.pot(), self.kin(), self.total_energy()
        )
    }
}

impl Moon {
    fn new<S: Into<String>>(name: S, position: Vector) -> Self {
        Moon {
            name: name.into(),
            position: position,
            velocity: Vector::new(0, 0, 0)
        }
    }

    fn apply_gravity(&mut self, other: &Moon) {
        self.velocity = self.velocity + gravity(&self.position, &other.position);
    }

    fn step(&mut self) {
        self.position = self.position + self.velocity;
    }

    fn pot(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kin(&self) -> i64 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i64 {
        self.pot() * self.kin()
    }

    fn x(&self) -> (i64, i64) {
        (self.position.x, self.velocity.x)
    }

    fn y(&self) -> (i64, i64) {
        (self.position.y, self.velocity.y)
    }

    fn z(&self) -> (i64, i64) {
        (self.position.z, self.velocity.z)
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
struct System {
    moons: Vec<Moon>
}

impl System {
    fn new() -> Self {
        System { moons: Vec::new() }
    }

    fn add_moon(&mut self, moon: Moon) {
        self.moons.push(moon);
    }

    fn step(&mut self) {
        self.moons.iter_mut()
            .map(|x| RefCell::new(x))
            // no clue what the fuck that is
            // just add shit until the compiler shuts the fuck up
            .collect::<Vec<RefCell<&mut Moon>>>()
            .iter()
            .tuple_combinations()
            .for_each(|(a, b)| {
                a.borrow_mut().apply_gravity(&b.borrow());
                b.borrow_mut().apply_gravity(&a.borrow());
            });

        self.moons.iter_mut().for_each(|moon| moon.step());
     }

    fn total_energy(&self) -> i64 {
        self.moons.iter().map(|moon| moon.total_energy()).sum()
    }

    fn moon_by_name<'a>(&'a self, name: &String) -> Option<&'a Moon> {
        self.moons.iter().filter(|moon| moon.name == *name).nth(0)
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.moons.iter()
            .sorted_by_key(|moon| moon.name.clone())
            .join("\n");

        write!(f, "----- Total Energy: {}\n{}\n-----", self.total_energy(), r)
    }
}


fn gravity(object: &Vector, towards: &Vector) -> Vector {
    let d = towards - object;
    Vector::new(
        if d.x == 0 { 0 } else { d.x.signum() as i64 },
        if d.y == 0 { 0 } else { d.y.signum() as i64 },
        if d.z == 0 { 0 } else { d.z.signum() as i64 }
    )
}


#[test]
fn test_gravity() {
    assert_eq!(gravity(&Vector::new(-1, 2, 3), &Vector::new(1, 2, -5)), Vector::new(1, 0, -1));
}


struct Solver {
    original: System,
    solution: (Option<usize>, Option<usize>, Option<usize>)
}

impl Solver {
    fn new(original: System) -> Self {
        Solver { original: original, solution: (None, None, None) }
    }

    fn solve(&mut self) -> usize {
        let mut steps = 1;
        let mut system = self.original.clone();

        while !self.is_solved() {
            system.step();
            self.feed(steps, &system);
            steps = steps + 1;
        }

        self.solution().unwrap()
    }

    fn feed(&mut self, time: usize, system: &System) {
        let moons = self.original.moons.iter()
            .map(|om| (om, system.moon_by_name(&om.name).unwrap()))
            .collect();

        Solver::do_feed(time, &moons, &mut self.solution.0, |moon| moon.x());
        Solver::do_feed(time, &moons, &mut self.solution.1, |moon| moon.y());
        Solver::do_feed(time, &moons, &mut self.solution.2, |moon| moon.z());
    }

    fn do_feed<F, T>(time: usize, moons: &Vec<(&Moon, &Moon)>, result: &mut Option<usize>, ex: F)
        where F: Fn(&Moon) -> T,
              T: PartialEq
    {
        if result.is_none() {
            let x = moons.iter()
                .all(|(om, nm)| ex(om) == ex(nm));

            if x {
                result.replace(time);
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.solution.0.is_some() && self.solution.1.is_some() && self.solution.2.is_some()
    }

    fn solution(&self) -> Option<usize> {
        match self.solution  {
            (Some(x), Some(y), Some(z)) => Some(x.lcm(&y.lcm(&z))),
            _ => None
        }
    }

}


fn nth_step(nth: usize, mut system: System) -> System {
    for _ in 0..nth {
        system.step();
    }
    system
}


fn main() {
    // Example 1
    let mut s1 = System::new();
    s1.add_moon(Moon::new("Callisto", Vector::new( -1,   0,   2)));
    s1.add_moon(Moon::new("Europa",   Vector::new(  2, -10,  -7)));
    s1.add_moon(Moon::new("Ganymede", Vector::new(  4,  -8,   8)));
    s1.add_moon(Moon::new("Io",       Vector::new(  3,   5,  -1)));

    // Example 2
    let mut s2 = System::new();
    s2.add_moon(Moon::new("Callisto", Vector::new( -8,  -10,   0)));
    s2.add_moon(Moon::new("Europa",   Vector::new(  5,    5,  10)));
    s2.add_moon(Moon::new("Ganymede", Vector::new(  2,   -7,   3)));
    s2.add_moon(Moon::new("Io",       Vector::new(  9,   -8,  -3)));

    // Input
    let mut s3 = System::new();
    s3.add_moon(Moon::new("Callisto", Vector::new( -1,  -4,   0)));
    s3.add_moon(Moon::new("Europa",   Vector::new(  4,   7,  -1)));
    s3.add_moon(Moon::new("Ganymede", Vector::new(-14, -10,   9)));
    s3.add_moon(Moon::new("Io",       Vector::new(  1,   2,  17)));

    let system = s3;

    println!("Step: 1000\n{}\n", nth_step(1000, system.clone()));

    let mut solver = Solver::new(system.clone());
    println!("Cycle after: {}", solver.solve());
}

