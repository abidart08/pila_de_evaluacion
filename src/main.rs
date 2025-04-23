// Importa las funcionalidades necesarias para manejar la entrada estándar
use std::io::{self, BufRead};

// Define una enumeración que representa las posibles instrucciones que el intérprete puede ejecutar
#[derive(Debug)]
enum Instruction {
    Ldc(f64), // Carga un número en la pila
    Neg,      // Cambia el signo del número en el tope de la pila
    Add,      // Suma los dos números en el tope de la pila
    Sub,      // Resta los dos números en el tope de la pila
    Mul,      // Multiplica los dos números en el tope de la pila
    Div,      // Divide los dos números en el tope de la pila
    Ceq,      // Compara si los dos números en el tope de la pila son iguales
    Cgt,      // Compara si el penúltimo número es mayor que el último en la pila
    Clt,      // Compara si el penúltimo número es menor que el último en la pila
    Dup,      // Duplica el número en el tope de la pila
    Pop,      // Elimina el número en el tope de la pila
}

// Función que convierte un token de texto en una instrucción
fn parse_texto(token: &str) -> Option<Instruction> {
    // Si el token comienza con "ldc:", intenta parsear el número que sigue
    if token.starts_with("ldc:") {
        let value = token[4..].parse::<f64>().ok()?; // Extrae el número después de "ldc:"
        Some(Instruction::Ldc(value)) // Devuelve la instrucción Ldc con el valor
    } else {
        // Intenta coincidir el token con una de las instrucciones conocidas
        match token {
            "neg" => Some(Instruction::Neg),
            "add" => Some(Instruction::Add),
            "sub" => Some(Instruction::Sub),
            "mul" => Some(Instruction::Mul),
            "div" => Some(Instruction::Div),
            "ceq" => Some(Instruction::Ceq),
            "cgt" => Some(Instruction::Cgt),
            "clt" => Some(Instruction::Clt),
            "dup" => Some(Instruction::Dup),
            "pop" => Some(Instruction::Pop),
            _ => None, // Si no coincide con ninguna instrucción, devuelve None
        }
    }
}

// Función que ejecuta una lista de instrucciones sobre una pila
fn execute(instructions: Vec<Instruction>, stack: &mut Vec<f64>) {
    // Itera sobre cada instrucción
    for instr in instructions {
        match instr {
            // Carga un número en la pila
            Instruction::Ldc(n) => stack.push(n),
            // Cambia el signo del número en el tope de la pila
            Instruction::Neg => {
                if let Some(x) = stack.pop() {
                    stack.push(-x);
                }
            }
            // Suma los dos números en el tope de la pila
            Instruction::Add => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                }
            }
            // Resta los dos números en el tope de la pila
            Instruction::Sub => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                }
            }
            // Multiplica los dos números en el tope de la pila
            Instruction::Mul => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                }
            }
            // Divide los dos números en el tope de la pila
            Instruction::Div => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a / b);
                }
            }
            // Compara si los dos números en el tope de la pila son iguales
            Instruction::Ceq => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(if a == b { 1.0 } else { 0.0 });
                }
            }
            // Compara si el penúltimo número es mayor que el último en la pila
            Instruction::Cgt => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(if a > b { 1.0 } else { 0.0 });
                }
            }
            // Compara si el penúltimo número es menor que el último en la pila
            Instruction::Clt => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(if a < b { 1.0 } else { 0.0 });
                }
            }
            // Duplica el número en el tope de la pila
            Instruction::Dup => {
                if let Some(x) = stack.last() {
                    stack.push(*x);
                }
            }
            // Elimina el número en el tope de la pila
            Instruction::Pop => {
                stack.pop();
            }
        }
    }
}

// Función principal del programa
fn main() {
    // Obtiene un manejador para leer la entrada estándar
    let stdin = io::stdin();
    // Inicializa una pila vacía
    let mut stack: Vec<f64> = Vec::new();

    // Lee las líneas de la entrada estándar
    for line in stdin.lock().lines() {
        let line = line.unwrap(); // Desempaqueta la línea leída
        // Convierte la línea en una lista de instrucciones
        let instructions: Vec<Instruction> = line
            .split_whitespace() // Divide la línea en tokens
            .filter_map(parse_texto) // Convierte cada token en una instrucción
            .collect(); // Recoge las instrucciones en un vector

        // Ejecuta las instrucciones sobre la pila
        execute(instructions, &mut stack);
        // Imprime el estado actual de la pila
        println!("{:?}", stack);
    }
}