mod stack;
use stack::{
    stack_calculator::StackCalculator,
    utils::{first, read_command_args},
};

use crate::stack::{helper::expression_parser::parse_expression, stack_calculator::StackElement};

fn main() {
    let stack_calculator: StackCalculator = StackCalculator::new();

    if let Some(equation) = first(&read_command_args()) {
        let parsed_expression: Vec<StackElement> = parse_expression(equation).unwrap_or(vec![]);
        let postfix_result: Vec<StackElement> = stack_calculator
            .populate_stack_with_parsed_expiression(parsed_expression)
            .infix_to_postfix();

        println!("\x1b[93m{:?}\x1b[0m", postfix_result)
    } else {
        println!("No arguments provided")
    }
}
