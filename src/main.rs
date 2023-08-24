mod stack;

use stack::{stack_calculator::StackCalculator, utils::read_command_args};

use crate::stack::{helper::expression_parser::parse_expression, stack_calculator::StackElement};

fn main() {
    let stack_calculator: StackCalculator = StackCalculator::new();
    if let Some(equation) = read_command_args().first() {
        let parsed_expression: Vec<StackElement> =
            parse_expression(equation).unwrap_or_else(|_| vec![]);
        let postfix_result = stack_calculator
            .populate_stack_with_parsed_expression(parsed_expression)
            .infix_to_postfix()
            .evaluate();

        match postfix_result {
            Ok(res) => println!("Result: {}", &res.get(0).unwrap()),
            Err(err) => println!("{}",err),
        }
    } else {
        println!("No arguments provided")
    }
}
