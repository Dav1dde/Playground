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

    pub fn run(&mut self) -> Option<i32> {
        let mut result = None;
        while !self.is_done() {
            let opcode = OpCode::read(self);
            result = opcode.execute(self).or(result);
        }

        result
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

    pub fn is_done(&self) -> bool {
        self.done
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
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let value = input.trim().parse().unwrap();

                let r = program.read(ParamMode::Immediate);
                program.write(r as usize, value);
                None
            }
            OpCode::Output(param_mode) => {
                let a = program.read(ParamMode::parse(*param_mode, 0));
                println!("output: {}", a);
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


fn main() {
    let mut program = Program::from_file(&mut File::open("../input.txt").unwrap()).unwrap();

    let result = program.run();
    println!("Result: {:?}", result);
}

