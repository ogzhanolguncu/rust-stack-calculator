pub mod parser_helper {
    use crate::stack_calculator::{Expressions, StackElement};
    use std::env;

    pub fn parse_expression(equation: &str) -> Result<Vec<StackElement>, &'static str> {
    let mut current_number = String::new();
    let mut stack_calculator: Vec<StackElement> = Vec::new();

    let push_number_if_any = |current_number: &mut String, stack_calculator: &mut Vec<StackElement>| {
        if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<i32>() {
                stack_calculator.push(StackElement::Operand(num));
                current_number.clear();
            }
        }
    };

    for element in equation.chars() {
        if element.is_whitespace() {
            continue;
        }

        if element.is_numeric() {
            current_number.push(element);
        } else {
            push_number_if_any(&mut current_number, &mut stack_calculator);

            let operator = match element {
                '+' => Expressions::Add,
                '-' => Expressions::Subtract,
                '/' => Expressions::Divide,
                '*' => Expressions::Multiply,
                '(' => Expressions::LeftParan,
                ')' => Expressions::RightParan,
                _ => return Err("Unknown operator"),
            };

            stack_calculator.push(StackElement::Operator(operator));
        }
    }

    push_number_if_any(&mut current_number, &mut stack_calculator);

    if stack_calculator.is_empty() {
        return Err("Something went wrong; expression might be missing.");
    }
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
