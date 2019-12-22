use std::io::prelude::*;
use std::io;
use std::num;
use std::fs::File;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cell::{RefCell};


#[derive(Debug)]
enum ParseError {
    IoError(io::Error),
    DataError(num::ParseIntError)
}


#[derive(Debug)]
enum RunResult {
    Paused(Option<i32>),
    Done(Option<i32>)
}

impl RunResult {
    fn unwrap(&self) -> i32 {
        match *self {
            RunResult::Paused(r) => r.unwrap(),
            RunResult::Done(r) => r.unwrap()
        }
    }
}

struct Program {
    position: usize,
    data: Vec<i32>,
    done: bool,
    paused: bool,
    io_handler: Option<Box<dyn IoHandler>>
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Program {
            position: self.position,
            data: self.data.clone(),
            done: self.done,
            paused: self.paused,
            io_handler: None
        }
    }
}

impl Program {
    pub fn from_file(file: &mut File) -> Result<Program, ParseError> {
        let mut data = String::new();
        file.read_to_string(&mut data).map_err(|x| ParseError::IoError(x))?;

        let opcodes: Result<Vec<i32>, _> = data.trim().split(",")
            .map(|x| x.parse::<i32>().map_err(|x| ParseError::DataError(x)))
            .collect();

        Ok(Program::from_opcodes(opcodes?))
    }

    pub fn from_opcodes(opcodes: Vec<i32>) -> Program {
        Program {
            position: 0,
            data: opcodes,
            done: false,
            paused: false,
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

    pub fn read(&mut self, mode: ParamMode) -> i32 {
        self.position += 1;
        match mode {
            ParamMode::Immediate => self.data[self.position - 1],
            ParamMode::Position => {
                let position = self.data[self.position - 1];
                self.data[position as usize]
            }
        }
    }

    pub fn write(&mut self, position: usize, value: i32) {
        self.data[position] = value;
    }

    pub fn jump(&mut self, position: usize) {
        self.position = position;
    }

    pub fn exit(&mut self) {
        self.done = true;
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

    pub fn output(&mut self, output: i32) {
        self.io_handler.as_mut().expect("expected io handler").output(output);
        self.pause();
    }

    pub fn input(&mut self) -> i32 {
        self.io_handler.as_mut().expect("expected io handler").input()
    }
}


#[derive(Debug)]
enum OpCode {
    Add(i32),
    Multiply(i32),
    Input,
    Output(i32),
    JumpIfTrue(i32),
    JumpIfFalse(i32),
    LessThan(i32),
    Equals(i32),
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
            3 => OpCode::Input,
            4 => OpCode::Output(param_mode),
            5 => OpCode::JumpIfTrue(param_mode),
            6 => OpCode::JumpIfFalse(param_mode),
            7 => OpCode::LessThan(param_mode),
            8 => OpCode::Equals(param_mode),
            99 => OpCode::Exit,
            _ => panic!("invalid opcode {}", opcode),
        }
    }

    fn execute(&self, program: &mut Program) -> Option<i32> {
        // println!("opcode: {:?}", self);
        match self {
            OpCode::Add(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read(ParamMode::Immediate);
                let result = a1 + a2;
                // println!("[{}] {} + {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r as usize, a1 + a2);
                Some(result)
            }
            OpCode::Multiply(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read(ParamMode::Immediate);
                let result = a1 * a2;
                // println!("[{}] {} * {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r as usize, a1 * a2);
                Some(result)
            }
            OpCode::Input => {
                let value = program.input();
                let r = program.read(ParamMode::Immediate);
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
                if a1 != 0 {
                    program.jump(a2);
                }
                None
            }
            OpCode::JumpIfFalse(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1)) as usize;
                if a1 == 0 {
                    program.jump(a2);
                }
                None
            }
            OpCode::LessThan(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read(ParamMode::Immediate) as usize;
                let result = if a1 < a2 { 1 } else { 0 };
                // println!("[{}] {} < {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r,  result);
                Some(result)
            }
            OpCode::Equals(param_mode) => {
                let a1 = program.read(ParamMode::parse(*param_mode, 0));
                let a2 = program.read(ParamMode::parse(*param_mode, 1));
                let r = program.read(ParamMode::Immediate) as usize;
                let result = if a1 == a2 { 1 } else { 0 };
                // println!("[{}] {} == {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r,  result);
                Some(result)
            }
            OpCode::Exit => {
                program.exit();
                None
            }
        }
    }
}


enum ParamMode {
    Position,
    Immediate
}

impl ParamMode {
    pub fn from_number(num: i32) -> Self {
        match num {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!("invalid param mode {}", num)
        }
    }

    pub fn parse(num: i32, position: u32) -> Self {
        ParamMode::from_number((num / i32::pow(10, position)) % 10)
    }
}


trait IoHandler {
    fn input(&mut self) -> i32;
    fn output(&mut self, value: i32);
}

struct StdInOutIoHandler {
}

impl IoHandler for StdInOutIoHandler {
    fn input(&mut self) -> i32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return input.trim().parse().unwrap();
    }

    fn output(&mut self, value: i32) {
        println!("-> {}", value);
    }
}

impl StdInOutIoHandler {
    fn new() -> Self {
        StdInOutIoHandler {}
    }
}


struct FixedIoHandler {
    input: Vec<i32>,
    output: Vec<i32>
}

impl IoHandler for FixedIoHandler {
    fn input(&mut self) -> i32 {
        self.input.remove(0)
    }

    fn output(&mut self, value: i32) {
        self.output.push(value);
    }
}

impl FixedIoHandler {
    fn new(input: Vec<i32>) -> Self {
        FixedIoHandler { input: input, output: Vec::new() }
    }
}


struct CombinationsIter {
    len: usize,
    current: usize,
    max: usize
}

impl Iterator for CombinationsIter {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= usize::pow(self.max, self.len as u32) {
            return None;
        }

        let mut numbers = Vec::new();

        for i in 0..self.len {
            let next = self.current / usize::pow(self.max, i as u32) % self.max;
            numbers.push(next as i32);
        }

        self.current = self.current + 1;

        Some(numbers)
    }
}

impl CombinationsIter {
    fn new(len: usize, max: usize) -> Self {
        CombinationsIter {
            len: len,
            current: 0,
            max: max
        }
    }
}


fn thruster(program: &Program, phase_settings: Vec<i32>) -> i32 {
    let mut data = 0;
    for phase_setting in phase_settings.clone() {
        let mut work = program.clone();
        work.set_io_handler(Box::new(FixedIoHandler::new(vec![phase_setting, data])));

        data = work.run().unwrap();
    }

    data
}

fn thruster_feedback(program: &Program, phase_settings: Vec<i32>) -> i32 {
    let mut data = 0;

    let programs: Vec<RefCell<Program>> = phase_settings.clone().iter()
        .map(|_| RefCell::new(program.clone()))
        .collect();

    let r = phase_settings.iter()
        .map(|x| x + 5)
        .zip(programs.iter())
        .cycle();

    let mut runs = 0;
    for (phase_setting, program) in r {
        let mut program = program.borrow_mut();

        let input = if runs < phase_settings.len() {
            vec![phase_setting, data]
        } else {
            vec![data]
        };

        program.set_io_handler(Box::new(FixedIoHandler::new(input)));
        program.resume();

        let result = program.run();
        if program.is_done() {
            break;
        }

        data = result.unwrap();

        runs = runs + 1;
    }

    data
}


fn main() {
    let program = Program::from_file(&mut File::open("../input.txt").unwrap()).unwrap();

    let max = CombinationsIter::new(5, 5)
        .filter(|x| !has_duplicates(x))
        .map(|x| thruster(&program, x))
        .max();
    println!("maximum thrust: {:?}", max);

    let max = CombinationsIter::new(5, 5)
        .filter(|x| !has_duplicates(x))
        .map(|x| thruster_feedback(&program, x))
        .max();
    println!("maximum thrust with feedback: {:?}", max);
}


fn has_duplicates(inp: &Vec<i32>) -> bool {
    let x: HashSet<i32> = HashSet::from_iter(inp.iter().cloned());
    x.len() < inp.len()
}

