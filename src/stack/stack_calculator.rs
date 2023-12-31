use super::{
    error::ParseCustomError,
    stack_calculator_utils::{convert_operators_to_postfix, convert_right_paren_to_postfix},
};

#[derive(PartialEq, Debug)]
pub enum StackElement {
    Operand(i32),
    Operator(Token),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub struct StackCalculator {
    stack: Vec<StackElement>,
}

impl StackCalculator {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn populate_stack_with_parsed_expression(
        &self,
        parsed_expression: Vec<StackElement>,
    ) -> Self {
        Self {
            stack: parsed_expression,
        }
    }

    pub fn infix_to_postfix(&self) -> Self {
        let mut operand_stack: Vec<StackElement> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        for item in &self.stack {
            match item {
                StackElement::Operand(operand) => {
                    operand_stack.push(StackElement::Operand(*operand))
                }
                StackElement::Operator(operator) => match operator {
                    Token::Add | Token::Subtract | Token::Multiply | Token::Divide => {
                        convert_operators_to_postfix(
                            &mut operator_stack,
                            operator,
                            &mut operand_stack,
                        );
                    }
                    Token::LeftParen => operator_stack.push(Token::LeftParen),
                    Token::RightParen => {
                        convert_right_paren_to_postfix(&mut operator_stack, &mut operand_stack)
                    }
                },
            }
        }

        operand_stack.extend(operator_stack.into_iter().rev().map(StackElement::Operator));

        Self {
            stack: operand_stack,
        }
    }

    pub fn evaluate(&self) -> Result<Vec<i32>, ParseCustomError> {
        let mut inner_stack: Vec<i32> = Vec::new();
        for item in &self.stack {
            match item {
                StackElement::Operand(operand) => inner_stack.push(*operand),
                StackElement::Operator(operator) => {
                    if inner_stack.len() < 2 {
                        return Err(ParseCustomError::NotEnoughArguments);
                    }
                    let n1 = inner_stack.pop().unwrap();
                    let n2 = inner_stack.pop().unwrap();

                    match operator {
                        Token::Add => inner_stack.push(n1 + n2),
                        Token::Subtract => inner_stack.push(n1 - n2),
                        Token::Multiply => inner_stack.push(n1 * n2),
                        Token::Divide => {
                            if n1 == 0 {
                                return Err(ParseCustomError::DivisionByZero);
                            }
                            inner_stack.push(n2 / n1) // Note: this depends on your stack semantics
                        }
                        err => return Err(ParseCustomError::UnknownToken(err.clone())),
                    }
                }
            }
        }
        Ok(inner_stack)
    }
}

#[cfg(test)]
mod stack_calculator_test {

    use crate::stack::{error::ParseCustomError, helper::expression_parser::parse_expression};

    use super::*;

    #[test]
    fn should_test_parse_from_infix_to_postfix() {
        let input = "5 + (6 * 8)";
        let stack_calculator: StackCalculator = StackCalculator::new();

        let parsed_expression: Vec<StackElement> = parse_expression(input).unwrap_or(vec![]);
        let postfix_result_variation = stack_calculator
            .populate_stack_with_parsed_expression(parsed_expression)
            .infix_to_postfix();

        assert_eq!(
            postfix_result_variation.stack,
            [
                StackElement::Operand(5),
                StackElement::Operand(6),
                StackElement::Operand(8),
                StackElement::Operator(Token::Multiply),
                StackElement::Operator(Token::Add)
            ]
        )
    }

    #[test]
    fn should_not_throw_when_dived_by_zero() {
        let input = "6 / 0";
        let stack_calculator: StackCalculator = StackCalculator::new();

        let parsed_expression: Vec<StackElement> = parse_expression(input).unwrap_or(vec![]);
        let postfix_result_variation = stack_calculator
            .populate_stack_with_parsed_expression(parsed_expression)
            .infix_to_postfix()
            .evaluate();

        assert_eq!(
            postfix_result_variation.unwrap_err(),
            ParseCustomError::DivisionByZero
        )
    }

    #[test]
    fn should_return_empty_when_input_is_empty() {
        let input = "";
        let stack_calculator: StackCalculator = StackCalculator::new();

        let parsed_expression: Vec<StackElement> = parse_expression(input).unwrap_or(vec![]);
        let postfix_result_variation = stack_calculator
            .populate_stack_with_parsed_expression(parsed_expression)
            .infix_to_postfix();

        assert_eq!(postfix_result_variation.stack, [])
    }

    #[test]
    fn should_return_correct_answer() {
        let input = "(3 * 4) + (5 * 2)";
        let stack_calculator: StackCalculator = StackCalculator::new();

        let parsed_expression: Vec<StackElement> = parse_expression(input).unwrap_or(vec![]);
        let postfix_result_variation = stack_calculator
            .populate_stack_with_parsed_expression(parsed_expression)
            .infix_to_postfix()
            .evaluate();

        assert_eq!(*postfix_result_variation.unwrap().first().unwrap(), 22)
    }

    #[test]
    fn should_return_error_for_unknown_operator() {
        let input = "5 $ ";
        let result = parse_expression(input);
        assert_eq!(result.unwrap_err(), ParseCustomError::UnknownOperator('$'));
    }

    #[test]
    fn should_return_error_for_empty_expression() {
        let input = "";
        let result = parse_expression(input);
        assert_eq!(result.unwrap_err(), ParseCustomError::EmptyExpression);
    }

    #[test]
    fn should_return_error_for_not_enough_arguments() {
        let input = "+";
        let result = parse_expression(input);
        assert_eq!(result.unwrap_err(), ParseCustomError::NotEnoughArguments);
    }
}
