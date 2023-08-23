pub mod expression_parser {
    use crate::stack::{
        error::ParseCustomError,
        stack_calculator::{StackElement, Token},
    };

    pub fn parse_expression(equation: &str) -> Result<Vec<StackElement>, ParseCustomError> {
        let mut stack: Vec<StackElement> = Vec::new();
        let mut current_number: String = String::new();

        for c in equation.chars() {
            if c.is_whitespace() {
                continue;
            }

            if c.is_numeric() {
                current_number.push(c);
                continue;
            }

            if !current_number.is_empty() {
                push_number_to_stack(&current_number, &mut stack);
                current_number.clear();
            }

            let operator = match c {
                '+' => Token::Add,
                '-' => Token::Subtract,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                unknown_char => return Err(ParseCustomError::UnknownOperator(unknown_char)),
            };
            stack.push(StackElement::Operator(operator));
        }

        if !current_number.is_empty() {
            push_number_to_stack(&current_number, &mut stack);
        }

        if stack.is_empty() {
            return Err(ParseCustomError::EmptyExpression);
        }

        if stack.len() < 3 {
            return Err(ParseCustomError::NotEnoughArguments);
        }

        Ok(stack)
    }

    fn push_number_to_stack(current_number: &str, stack: &mut Vec<StackElement>) {
        let num = current_number.parse::<i32>().unwrap();
        stack.push(StackElement::Operand(num));
    }

    pub fn precedence(op: &Token) -> i32 {
        match op {
            Token::Add | Token::Subtract => 1,
            Token::Multiply | Token::Divide => 2,
            _ => 0,
        }
    }
}
