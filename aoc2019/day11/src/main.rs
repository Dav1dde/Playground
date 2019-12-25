use std::io;
use std::num;
use std::fmt;
use std::cmp;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


#[derive(Debug)]
enum ParseError {
    IoError(io::Error),
    DataError(num::ParseIntError)
}


#[derive(Debug)]
enum RunResult {
    Paused(Option<i64>),
    Done(Option<i64>)
}

impl RunResult {
    fn unwrap(&self) -> i64 {
        match *self {
            RunResult::Paused(r) => r.unwrap(),
            RunResult::Done(r) => r.unwrap()
        }
    }
}

struct Program {
    position: usize,
    data: Vec<i64>,
    done: bool,
    paused: bool,
    ram: HashMap<usize, i64>,
    relative_base: i64,
    io_handler: Option<Box<dyn IoHandler>>
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Program {
            position: self.position,
            data: self.data.clone(),
            done: self.done,
            paused: self.paused,
            ram: self.ram.clone(),
            relative_base: self.relative_base,
            io_handler: None
        }
    }
}

impl Program {
    pub fn from_file(file: &mut File) -> Result<Self, ParseError> {
        let mut data = String::new();
        file.read_to_string(&mut data).map_err(|x| ParseError::IoError(x))?;

        let opcodes: Result<Vec<i64>, _> = data.trim().split(",")
            .map(|x| x.parse::<i64>().map_err(|x| ParseError::DataError(x)))
            .collect();

        Ok(Program::from_opcodes(opcodes?))
    }

    pub fn from_opcodes(opcodes: Vec<i64>) -> Self {
        Program {
            position: 0,
            data: opcodes,
            done: false,
            paused: false,
            ram: HashMap::new(),
            relative_base: 0,
            io_handler: None
        }
    }

    pub fn set_io_handler(&mut self, io_handler: Box<dyn IoHandler>) {
        self.io_handler = Some(io_handler);
    }

    pub fn run(&mut self) -> RunResult {
        let mut result = None;
        while self.is_running() {
            let opcode = OpCode::read(self);
            result = opcode.execute(self).or(result);
        }

        if self.is_done() { RunResult::Done(result) } else { RunResult::Paused(result) }
    }

    pub fn read(&mut self, mode: ParamMode) -> i64 {
        let position = self.data[self.position];
        self.position = self.position + 1;

        match mode {
            ParamMode::Immediate => position,
            ParamMode::Position => self.read_internal(position as usize),
            ParamMode::Relative => self.read_internal((self.relative_base + position) as usize)
        }
    }

    pub fn read_pos(&mut self, mode: ParamMode) -> usize {
        let position = self.data[self.position];
        self.position = self.position + 1;

        match mode {
            ParamMode::Immediate => position as usize,
            ParamMode::Position => position as usize,
            ParamMode::Relative => (self.relative_base + position) as usize
        }
    }

    fn read_internal(&self, index: usize) -> i64 {
        if index < self.data.len() {
            self.data[index]
        } else {
            *self.ram.get(&index).unwrap_or(&0)
        }
    }

    pub fn write(&mut self, position: usize, value: i64) {
        if position < self.data.len() {
            self.data[position] = value;
        } else {
            self.ram.insert(position, value);
        }
    }

    pub fn jump(&mut self, position: usize) {
        self.position = position;
    }

    pub fn adjust_relative_base(&mut self, relative_base: i64) {
        self.relative_base = self.relative_base + relative_base;
    }

    pub fn exit(&mut self) {
        self.done = true;
        self.io_handler.as_mut().expect("expected io handler").done();
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn is_running(&self) -> bool {
        !self.done && !self.paused
    }

    pub fn output(&mut self, output: i64) {
        self.io_handler.as_mut().expect("expected io handler").output(output);
        // self.pause();
    }

    pub fn input(&mut self) -> i64 {
        self.io_handler.as_mut().expect("expected io handler").input()
    }
}


#[derive(Debug)]
enum OpCode {
    Add(i64),
    Multiply(i64),
    Input(i64),
    Output(i64),
    JumpIfTrue(i64),
    JumpIfFalse(i64),
    LessThan(i64),
    Equals(i64),
    RelativeBase(i64),
    Exit
}

impl OpCode {
    fn read(program: &mut Program) -> OpCode {
        let instruction = program.read(ParamMode::Immediate);
        let opcode = instruction % 100;
        let param_mode = instruction / 100;

        match opcode {
            1 => OpCode::Add(param_mode),
            2 => OpCode::Multiply(param_mode),
            3 => OpCode::Input(param_mode),
            4 => OpCode::Output(param_mode),
            5 => OpCode::JumpIfTrue(param_mode),
            6 => OpCode::JumpIfFalse(param_mode),
            7 => OpCode::LessThan(param_mode),
            8 => OpCode::Equals(param_mode),
            9 => OpCode::RelativeBase(param_mode),
            99 => OpCode::Exit,
            _ => panic!("invalid opcode {}", opcode),
        }
    }

    fn execute(&self, program: &mut Program) -> Option<i64> {
        // println!("opcode: {:?}", self);
        match self {
            OpCode::Add(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read_pos(ParamMode::parse(*param_mode, 2));
                let result = a1 + a2;
                // println!("[{}] {} + {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r as usize, a1 + a2);
                Some(result)
            }
            OpCode::Multiply(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read_pos(ParamMode::parse(*param_mode, 2));
                let result = a1 * a2;
                // println!("[{}] {} * {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r as usize, a1 * a2);
                Some(result)
            }
            OpCode::Input(param_mode) => {
                let value = program.input();
                let r = program.read_pos(ParamMode::parse(*param_mode, 0));
                // println!("[{}] {} -> {}", program.position - 4, value, r);
                program.write(r as usize, value);
                None
            }
            OpCode::Output(param_mode) => {
                let a = program.read(ParamMode::parse(*param_mode, 0));
                program.output(a);
                None
            }
            OpCode::JumpIfTrue(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1)) as usize;
                // println!("[{}] {} != 0 jump {}", program.position - 4, a1, a2);
                if a1 != 0 {
                    program.jump(a2);
                }
                None
            }
            OpCode::JumpIfFalse(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1)) as usize;
                // println!("[{}] {} == 0 jump {}", program.position - 4, a1, a2);
                if a1 == 0 {
                    program.jump(a2);
                }
                None
            }
            OpCode::LessThan(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read_pos(ParamMode::parse(*param_mode, 2)) as usize;
                let result = if a1 < a2 { 1 } else { 0 };
                // println!("[{}] {} < {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r,  result);
                Some(result)
            }
            OpCode::Equals(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read_pos(ParamMode::parse(*param_mode, 2)) as usize;
                let result = if a1 == a2 { 1 } else { 0 };
                // println!("[{}] {} == {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r,  result);
                Some(result)
            }
            OpCode::RelativeBase(param_mode) => {
                let a = program.read(ParamMode::parse(*param_mode, 0));
                // println!("[{}] relative base: {}", program.position - 4, a);
                program.adjust_relative_base(a);
                None
            }
            OpCode::Exit => {
                program.exit();
                None
            }
        }
    }
}


#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative
}

impl ParamMode {
    pub fn from_number(num: i64) -> Self {
        match num {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("invalid param mode {}", num)
        }
    }

    pub fn parse(num: i64, position: u32) -> Self {
        ParamMode::from_number((num / i64::pow(10, position)) % 10)
    }
}


trait IoHandler {
    fn input(&mut self) -> i64;
    fn output(&mut self, value: i64);

    fn done(&mut self) {
    }
}

struct StdInOutIoHandler {
}

impl IoHandler for StdInOutIoHandler {
    fn input(&mut self) -> i64 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return input.trim().parse().unwrap();
    }

    fn output(&mut self, value: i64) {
        println!("-> {}", value);
    }
}

impl StdInOutIoHandler {
    fn new() -> Self {
        StdInOutIoHandler {}
    }
}


struct FixedIoHandler {
    input: Vec<i64>,
    output: Vec<i64>
}

impl IoHandler for FixedIoHandler {
    fn input(&mut self) -> i64 {
        self.input.remove(0)
    }

    fn output(&mut self, value: i64) {
        self.output.push(value);
    }
}

impl FixedIoHandler {
    fn new(input: Vec<i64>) -> Self {
        FixedIoHandler { input: input, output: Vec::new() }
    }
}


#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left
        }
    }
}

#[derive(Debug)]
enum Color {
    Black,
    White
}

impl Color {
    fn from_number(num: i32) -> Self {
        match num {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("invalid color {}", num)
        }
    }

    fn to_number(&self) -> i32 {
        match self {
            Color::Black => 0,
            Color::White => 1
        }
    }
}


#[derive(Debug)]
enum State {
    ExpectColor,
    ExpectDirection
}

#[derive(Debug)]
struct PaintingIoHandler {
    direction: Direction,
    position: (i32, i32),
    tiles: HashMap<(i32, i32), Color>,
    state: State
}

impl IoHandler for PaintingIoHandler {
    fn input(&mut self) -> i64 {
        self.tiles.get(&self.position).unwrap_or(&Color::Black).to_number() as i64
    }

    fn output(&mut self, value: i64) {
        self.state = match self.state {
            State::ExpectColor => {
                let color = Color::from_number(value as i32);
                self.tiles.insert(self.position, color);

                State::ExpectDirection
            },
            State::ExpectDirection => {
                self.direction = match value {
                    0 => self.direction.turn_left(),
                    1 => self.direction.turn_right(),
                    _ => panic!("invalid direction {}", value)
                };

                self.position = match self.direction {
                    Direction::Left => (self.position.0 - 1, self.position.1),
                    Direction::Right => (self.position.0 + 1, self.position.1),
                    Direction::Up => (self.position.0, self.position.1 + 1),
                    Direction::Down => (self.position.0, self.position.1 - 1)
                };

                State::ExpectColor
            }
        };
    }

    fn done(&mut self) {
        println!("Tiles colored: {}", self.tiles.len());
        println!("{}\n", self);
    }
}


impl fmt::Display for PaintingIoHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut min = (0, 0);
        let mut max = (0, 0);
        for position in self.tiles.keys() {
            min = (cmp::min(position.0, min.0), cmp::min(position.1, min.1));
            max = (cmp::max(position.0, max.0), cmp::max(position.1, max.1));
        }
        let width = max.0 - min.0;
        let height = max.1 - min.1 + 1;

        let mut rows = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let position = (min.0 + x, min.1 + y);
                let value = match self.tiles.get(&position).unwrap_or(&Color::Black) {
                    Color::White => "\u{2588}",
                    Color::Black => " "
                };
                row.push(value);
            }
            rows.push(row.join(""));
        }

        // we begin with 0 height not max height, so need to flip here
        rows.reverse();

        write!(f, "{}", rows.join("\n"))
    }
}

impl PaintingIoHandler {
    fn new(color: Color) -> Self {
        let position = (0, 0);

        let mut tiles = HashMap::new();
        tiles.insert(position, color);

        PaintingIoHandler {
            direction: Direction::Up,
            position: position,
            tiles: tiles,
            state: State::ExpectColor
        }
    }
}


fn main() {
    let mut program = Program::from_file(&mut File::open("../input.txt").unwrap()).unwrap();
    program.set_io_handler(Box::new(PaintingIoHandler::new(Color::Black)));

    program.run();
}

