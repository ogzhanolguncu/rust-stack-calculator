use super::{stack_calculator::{Token, StackElement}, helper::expression_parser::precedence};

pub fn convert_operators_to_postfix(operator_stack: &mut Vec<Token>, operator: &Token, operand_stack: &mut Vec<StackElement>) {
    while operator_stack
        .last()
        .map_or(false, |top| precedence(top) >= precedence(operator))
    {
        if let Some(op) = operator_stack.pop() {
            operand_stack.push(StackElement::Operator(op));
        }
    }
    operator_stack.push(operator.clone());
}

pub fn convert_right_paren_to_postfix(
    operator_stack: &mut Vec<Token>,
    operand_stack: &mut Vec<StackElement>,
) {
    while let Some(top) = operator_stack.last() {
        if *top == Token::LeftParen {
            break;
        }
        if let Some(op) = operator_stack.pop() {
            operand_stack.push(StackElement::Operator(op));
        }
    }
    operator_stack.pop();
    // This will pop the LeftParan, if it exists
}
