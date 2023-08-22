use crate::helper::parser_helper::precedence;

#[derive(PartialEq, Debug)]
pub enum StackElement {
    Operand(i32),
    Operator(Expressions),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expressions {
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParan,
    RightParan,
}

#[derive(Debug)]
pub struct StackCalculator {
    stack: Vec<StackElement>,
}

impl StackCalculator {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn populate_stack_with_parsed_expiression(
        &self,
        parsed_expression: Vec<StackElement>,
    ) -> Self {
        Self {
            stack: parsed_expression,
        }
    }

    pub fn infix_to_postfix(&self) -> Vec<StackElement> {
        let mut operand_stack: Vec<StackElement> = Vec::new();
        let mut operator_stack: Vec<Expressions> = Vec::new();

        for item in &self.stack {
            match item {
                StackElement::Operand(operand) => {
                    operand_stack.push(StackElement::Operand(*operand))
                }
                StackElement::Operator(operator) => match operator {
                    Expressions::Add
                    | Expressions::Subtract
                    | Expressions::Multiply
                    | Expressions::Divide => {
                        while operator_stack
                            .last()
                            .map_or(false, |top| precedence(&top) >= precedence(operator))
                        {
                            if let Some(op) = operator_stack.pop() {
                                operand_stack.push(StackElement::Operator(op));
                            }
                        }
                        operator_stack.push(operator.clone());
                    }
                    Expressions::LeftParan => operator_stack.push(Expressions::LeftParan),
                    Expressions::RightParan => {
                        while let Some(top) = operator_stack.last() {
                            if *top == Expressions::LeftParan {
                                break;
                            }
                            if let Some(op) = operator_stack.pop() {
                                operand_stack.push(StackElement::Operator(op));
                            }
                        }
                        operator_stack.pop(); // This will pop the LeftParan, if it exists
                    }
                },
            }
        }

        while let Some(op) = operator_stack.pop() {
            operand_stack.push(StackElement::Operator(op));
        }

        operand_stack
    }
}

#[cfg(test)]
mod stack_calculator_test {
    use crate::helper::parser_helper::parse_expression;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_test_parse_from_infix_to_postfix() {
        let input = "5 + (6 * 8)";
        let stack_calculator: StackCalculator = StackCalculator::new();

        let parsed_expression: Vec<StackElement> = parse_expression(input).unwrap_or(vec![]);
        let postfix_result_variation = stack_calculator
            .populate_stack_with_parsed_expiression(parsed_expression)
            .infix_to_postfix();

        assert_eq!(
            postfix_result_variation,
            [
                StackElement::Operand(5),
                StackElement::Operand(6),
                StackElement::Operand(8),
                StackElement::Operator(Expressions::Multiply),
                StackElement::Operator(Expressions::Add)
            ]
        )
    }


    #[test]
    fn should_return_empty_when_input_is_empty() {
        let input = " ";
        let stack_calculator: StackCalculator = StackCalculator::new();

        let parsed_expression: Vec<StackElement> = parse_expression(input).unwrap_or(vec![]);
        let postfix_result_variation = stack_calculator
            .populate_stack_with_parsed_expiression(parsed_expression)
            .infix_to_postfix();

        assert_eq!(postfix_result_variation, [])
    }
}
