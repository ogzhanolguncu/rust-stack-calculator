pub mod parser_helper {
    use crate::stack_calculator::{Expressions, StackElement};
    use std::env;

    pub fn parse_equation(equation: &str) -> Result<Vec<StackElement>, &str> {
        let mut current_number = String::new();
        let mut stack_calculator: Vec<StackElement> = Vec::new();

        for element in equation.chars() {
            if element.is_whitespace() {
                continue;
            };
            if element.is_numeric() {
                current_number.push(element);
            } else {
                if !current_number.is_empty() {
                    let num = current_number.parse::<i32>().unwrap();
                    stack_calculator.push(StackElement::Operand(num));
                    current_number.clear();
                }

                let operator = match element {
                    '+' => Expressions::Add,
                    '-' => Expressions::Subtract,
                    '/' => Expressions::Divide,
                    '*' => Expressions::Multiply,
                    '(' => Expressions::LeftParan,
                    ')' => Expressions::RightParan,
                    _ => panic!("Unknown operator"),
                };

                stack_calculator.push(StackElement::Operator(operator));
            }
        }

        if current_number.is_empty() {
            return Err("Something went wrong number might be missing");
        }
        let num = current_number.parse::<i32>().unwrap();
        stack_calculator.push(StackElement::Operand(num));
        Ok(stack_calculator)
    }

    pub fn read_args() -> Vec<String> {
        env::args().skip(1).collect()
    }
    pub fn first<T>(v: &Vec<T>) -> Option<&T> {
        v.first()
    }

    pub fn precedence(op: &Expressions) -> i32 {
    match op {
        Expressions::Add | Expressions::Subtract => 1,
        Expressions::Multiply | Expressions::Divide => 2,
        _ => 0,
    }
}
}
