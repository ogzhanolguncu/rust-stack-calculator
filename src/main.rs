use crate::helper::parser_helper::parse_equation;
use helper::parser_helper::{first, read_args};
use stack_calculator::{StackCalculator, StackElement};

pub mod helper;
pub mod stack_calculator;
//TODO: WRITE SOME TEST TO COVER EXISTING CASES FIRST THEN CONTINUE WITH executing reverse postfix notation
fn main() {
    let input = read_args(); // Renamed to read_args to better describe what it does
    let stack_calculator: StackCalculator = StackCalculator::new();

    if let Some(equation) = first(&input) {
        let parsed_expression: Vec<StackElement> = parse_equation(equation).unwrap_or(vec![]);
        let postfix_result = stack_calculator
            .populate_stack_with_parsed_expiression(parsed_expression)
            .infix_to_postfix();
        println!("\x1b[93m{:?}\x1b[0m", postfix_result)
    } else {
        println!("No arguments provided")
    }
}
