use std::io::prelude::*;
use std::io;
use std::num;
use std::fs::File;


#[derive(Debug)]
enum ParseError {
    IoError(io::Error),
    DataError(num::ParseIntError)
}


#[derive(Debug, Clone)]
struct Program {
    position: usize,
    data: Vec<i32>,
    done: bool
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
            done: false
        }
    }

    pub fn run(&mut self) -> i32 {
        let mut result: i32 = -1;
        while !self.is_done() {
            let opcode = match self.read() {
                1 => OpCode::Add,
                2 => OpCode::Multiply,
                99 => OpCode::Exit,
                _ => panic!("invalid program"),
            };

            result = opcode.execute(self).unwrap_or(result);
        }

        result
    }

    pub fn read(&mut self) -> i32 {
        let result = self.data[self.position];
        self.position += 1;
        result
    }

    pub fn read_ref_value(&mut self) -> i32 {
        let position = self.read();
        self.data[position as usize]
    }

    pub fn write(&mut self, position: usize, value: i32) {
        self.data[position] = value;
    }

    pub fn exit(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}


#[derive(Debug)]
enum OpCode {
    Add,
    Multiply,
    Exit
}

impl OpCode {
    fn execute(&self, program: &mut Program) -> Option<i32> {
        match self {
            OpCode::Add => {
                let a1 = program.read_ref_value();
                let a2 = program.read_ref_value();
                let r = program.read();
                let result = a1 + a2;
                // println!("[{}] {} + {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r as usize, a1 + a2);
                Some(result)
            }
            OpCode::Multiply => {
                let a1 = program.read_ref_value();
                let a2 = program.read_ref_value();
                let r = program.read();
                let result = a1 * a2;
                // println!("[{}] {} * {} = {} -> {}", program.position - 4, a1, a2, result, r);
                program.write(r as usize, a1 * a2);
                Some(result)
            }
            OpCode::Exit => {
                program.exit();
                None
            }
        }
    }
}


fn find_noun_and_verb(program: &Program, target: i32) -> Option<i32> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut work = program.clone();
            work.write(1, noun);
            work.write(2, verb);
            let result = work.run();
            if result == target {
                return Some(noun * 100 + verb);
            }
        }
    }

    None
}


fn main() {
    let program = Program::from_file(&mut File::open("../input.txt").unwrap()).unwrap();

    let mut r1 = program.clone();
    r1.write(1, 12);
    r1.write(2, 2);
    let result = r1.run();
    println!("Result: {}", result);

    println!("Noun and Verb: {}", find_noun_and_verb(&program, 19690720).unwrap());
}


#[test]
fn test_program() {
    assert_eq!(Program::from_opcodes(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).run(), 3500);
    assert_eq!(Program::from_opcodes(vec![1, 0, 0, 0, 99]).run(), 2);
    assert_eq!(Program::from_opcodes(vec![2, 3, 0, 3, 99]).run(), 6);
    assert_eq!(Program::from_opcodes(vec![2, 4, 4, 5, 99, 0]).run(), 9801);
    assert_eq!(Program::from_opcodes(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).run(), 30);
}

