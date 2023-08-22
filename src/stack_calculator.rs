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
    pub fn populate_stack_with_parsed_expiression(&self,parsed_expression: Vec<StackElement>) -> Self {
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
