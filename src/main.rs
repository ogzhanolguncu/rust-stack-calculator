use std::env;

use stack_calculator::{Expressions, StackCalculator, StackElement};

pub mod stack_calculator;

fn main() {
    let input = read_args(); // Renamed to read_args to better describe what it does
    let mut stack_calculator = StackCalculator::new();

    if let Some(equation) = first(&input) {
        parse_equation(equation, &mut stack_calculator);
    } else {
        println!("No arguments provided")
    }
    println!("{}",stack_calculator.evaluate().unwrap_or(-1))
}

fn parse_equation(equation: &String, stack_calculator: &mut StackCalculator) {
    for element in equation.replace(" ", "").chars() {
        let is_element_numeric = element.is_numeric();
        if is_element_numeric {
            let num = element.to_string().parse::<i32>().unwrap();
            stack_calculator.push(StackElement::Number(num))
        } else {
            let operator = match element {
                '+' => Expressions::ADD,
                '-' => Expressions::SUBTRACT,
                '/' => Expressions::DIVIDE,
                '*' => Expressions::MULTIPLY,
                _ => panic!("Unknown operator"),
            };
            stack_calculator.push(StackElement::Operator(operator));
        }
    }
}

fn read_args() -> Vec<String> {
    env::args().skip(1).collect()
}
fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}
