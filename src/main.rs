use std::io::{self, BufRead};

#[derive(Debug)]
enum Instruction {
    Ldc(f64),
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

fn parse_instruction(token: &str) -> Option<Instruction> {
    if token.starts_with("ldc:") {
        let value = token[4..].parse::<f64>().ok()?;
        Some(Instruction::Ldc(value))
    } else {
        match token {
            "neg" => Some(Instruction::Neg),
            "add" => Some(Instruction::Add),
            "sub" => Some(Instruction::Sub),
            "mul" => Some(Instruction::Mul),
            "div" => Some(Instruction::Div),
            _ => None,
        }
    }
}

fn execute(instructions: Vec<Instruction>, stack: &mut Vec<f64>) {
    for instr in instructions {
        match instr {
            Instruction::Ldc(n) => stack.push(n),
            Instruction::Neg => {
                if let Some(x) = stack.pop() {
                    stack.push(-x);
                }
            }
            Instruction::Add => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                }
            }
            Instruction::Sub => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                }
            }
            Instruction::Mul => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                }
            }
            Instruction::Div => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a / b);
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stack: Vec<f64> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let instructions: Vec<Instruction> = line
            .split_whitespace()
            .filter_map(parse_instruction)
            .collect();

        execute(instructions, &mut stack);
        println!("{:?}", stack);
    }
}
