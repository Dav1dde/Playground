use std::ops;
use std::fs;
use std::collections::HashMap;

pub type CableId = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    x: i32,
    y: i32
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x: x, y: y }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32)
}


impl<'a, 'b> ops::Add<&'b Vector> for &'a Instruction {
    type Output = Vector;

    fn add(self, rhs: &'b Vector) -> Vector {
        match self {
            Instruction::Left(amount) => Vector::new(rhs.x - amount, rhs.y),
            Instruction::Right(amount) => Vector::new(rhs.x + amount, rhs.y),
            Instruction::Up(amount) => Vector::new(rhs.x, rhs.y + amount),
            Instruction::Down(amount) => Vector::new(rhs.x, rhs.y - amount)
        }
    }
}

impl Instruction {
    fn coordinates(&self, from: Vector) -> Vec<Vector> {
        let to = self + &from;
        let mut result = Vec::new();

        let x_mult = match self {
            Instruction::Left(_) => -1,
            Instruction::Right(_) => 1,
            _ => 0
        };
        let y_mult = match self {
            Instruction::Down(_) => -1,
            Instruction::Up(_) => 1,
            _ => 0
        };

        for x in 0..(to.x - from.x).abs() + 1 {
            result.push(Vector::new(from.x + (x * x_mult), from.y));
        }
        for y in 1..(to.y - from.y).abs() + 1 {
            result.push(Vector::new(to.x, from.y + (y * y_mult)));
        }

        result
    }
}

#[test]
fn test_instruction_coordinates() {
    assert_eq!(
        Instruction::Left(2).coordinates(Vector::new(1, 1)),
        vec![Vector::new(1, 1), Vector::new(0, 1), Vector::new(-1, 1)]
    );
    assert_eq!(
        Instruction::Right(2).coordinates(Vector::new(2, 1)),
        vec![Vector::new(2, 1), Vector::new(3, 1), Vector::new(4, 1)]
    );
    assert_eq!(
        Instruction::Down(2).coordinates(Vector::new(2, 1)),
        vec![Vector::new(2, 1), Vector::new(2, 0), Vector::new(2, -1)]
    );
    assert_eq!(
        Instruction::Up(2).coordinates(Vector::new(2, -1)),
        vec![Vector::new(2, -1), Vector::new(2, 0), Vector::new(2, 1)]
    );
}


pub type Path = Vec<Instruction>;

pub fn parse<I, T>(path: I) -> Path
    where I: IntoIterator<Item = T>,
          T: Into<String>
{
    let mut result = Vec::new();

    for instruction in path.into_iter() {
        let instruction = instruction.into();
        let (direction, amount) = instruction.split_at(1);
        let amount = amount.parse::<i32>().unwrap();

        result.push(
            match direction {
                "R" => Instruction::Right(amount),
                "L" => Instruction::Left(amount),
                "U" => Instruction::Up(amount),
                "D" => Instruction::Down(amount),
                _ => panic!("invalid instruction {}", instruction)
            }
        )
    }

    result
}

#[derive(Debug)]
pub struct Point {
    cables: HashMap<CableId, Vec<usize>>
}

impl Point {
    pub fn new() -> Self {
        Point { cables: HashMap::new() }
    }

    pub fn add(&mut self, cable: CableId, distance: usize) {
        self.cables.entry(cable).or_insert_with(|| Vec::new()).push(distance);
    }

    pub fn is_intersection(&self) -> bool {
        self.cables.len() > 1
    }

    pub fn min_distance(&self, cable: CableId) -> Option<usize> {
        self.cables.get(&cable).and_then(|x| x.iter().min()).map(|x| *x)
    }

    pub fn total_min_distance(&self) -> usize {
        self.cables.keys().filter_map(|x| self.min_distance(*x)).sum()
    }
}


#[derive(Debug)]
struct Board {
    central_port: Vector,
    data: HashMap<Vector, Point>
}


impl Board {
    pub fn new() -> Self {
        Board {
            central_port: Vector::new(0, 0),
            data: HashMap::new()
        }
    }

    pub fn add_cable(&mut self, cable_id: CableId, path: &Path) {
        let mut last = self.central_port.clone();
        let mut distance = 1;

        for instruction in path {
            // coordinates contains the first element (starting point)
            // --> skip first to not mess up intersections and distance
            for v in instruction.coordinates(last).into_iter().skip(1) {
                self.data.entry(v).or_insert_with(|| Point::new()).add(cable_id, distance);
                distance = distance + 1;
                last = v;
            }
        }
    }

    pub fn intersections(&self) -> impl Iterator<Item=(&Point, &Vector)> {
        self.data.iter()
            .filter(|(_, v)| v.is_intersection())
            .map(|(k, v)| (v, k))
    }

    pub fn closest_intersection<F>(&self, intersection_fun: F) -> Option<usize>
        where F: Fn(&Point, &Vector) -> usize
    {
        self.intersections()
            .map(|(point, v)| intersection_fun(point, v))
            .min()
    }
}


fn manhattan_distance(a: &Vector, b: &Vector) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}


fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    let mut board = Board::new();
    for (i, data) in input.lines().enumerate() {
        board.add_cable(i as i32, &parse(data.split(",")));
    }

    let mh = board.closest_intersection(|_, v| manhattan_distance(&board.central_port, v))
        .expect("at least one intersection");
    println!("Closest intersection manhattan distance: {:?}", mh);

    let closest = board.closest_intersection(|p, _| p.total_min_distance())
        .expect("at least one intersection");
    println!("Closest intersection: {:?}", closest);
}


#[test]
fn test_cable_intersection_1() {
    let mut board = Board::new();

    let cable1 = parse("R2".split(","));
    let cable2 = parse("U1,R1,D2".split(","));
    board.add_cable(1, &cable1);
    board.add_cable(2, &cable2);

    println!("board: {:#?}", board);

    assert_eq!(board.intersections().count(), 1);
    assert_eq!(
        board.closest_intersection(|_, v| manhattan_distance(&board.central_port, v)),
        Some(1)
    );
    assert_eq!(
        board.closest_intersection(|p, _| p.total_min_distance()),
        Some(4)
    );
}

#[test]
fn test_cable_intersection_2() {
    let mut board = Board::new();

    let cable1 = parse("R8,U5,L5,D3".split(","));
    let cable2 = parse("U7,R6,D4,L4".split(","));
    board.add_cable(1, &cable1);
    board.add_cable(2, &cable2);

    assert_eq!(board.intersections().count(), 2);
    assert_eq!(
        board.closest_intersection(|_, v| manhattan_distance(&board.central_port, v)),
        Some(6)
    );
}

#[test]
fn test_cable_intersection_3() {
    let mut board = Board::new();

    let cable1 = parse("R75,D30,R83,U83,L12,D49,R71,U7,L72".split(","));
    let cable2 = parse("U62,R66,U55,R34,D71,R55,D58,R83".split(","));
    board.add_cable(1, &cable1);
    board.add_cable(2, &cable2);

    assert_eq!(
        board.closest_intersection(|_, v| manhattan_distance(&board.central_port, v)),
        Some(159)
    );
    assert_eq!(
        board.closest_intersection(|p, _| p.total_min_distance()),
        Some(610)
    );
}

