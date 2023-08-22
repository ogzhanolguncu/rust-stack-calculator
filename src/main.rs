use stack_calculator::{StackCalculator, StackElement};
use crate::helper::parser_helper::parse_equation;
use helper::parser_helper::{first, read_args};

pub mod helper;
pub mod stack_calculator;
//TODO: WRITE SOME TEST TO COVER EXISTING CASES FIRST THEN CONTINUE WITH executing reverse postfix notation
fn main() {
    let input = read_args(); // Renamed to read_args to better describe what it does
    let mut stack_calculator = StackCalculator::new();

    if let Some(equation) = first(&input) {
        //TODO: this should directly return new Vec instead of mutating for better readability
        parse_equation(equation, &mut stack_calculator); //INFO: Parses equation into readable format from string input then turns them into StackElement and then mutates stack_calculator

        let postfix: Vec<StackElement> = stack_calculator.infix_to_postfix();
        println!("\x1b[93m{:?}\x1b[0m", postfix)
    } else {
        println!("No arguments provided")
    }
}
