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

    pub fn push(&mut self, elem: StackElement) {
        self.stack.push(elem);
    }

    pub fn evaluate(&self) -> Option<i32> {
        None
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

//TODO: tests are pretty much useless at this point gotta make them more robust
#[cfg(test)]
mod tests {
    use crate::stack_calculator::{Expressions, StackCalculator, StackElement};

    #[test]
    fn should_add_number_to_stack() {
        let mut calc = StackCalculator::new();
        calc.push(StackElement::Operand(5));
        assert_eq!(1, calc.stack.len());
    }

    #[test]
    fn should_add_expression_to_stack() {
        let mut calc = StackCalculator::new();
        calc.push(StackElement::Operand(5));
        calc.push(StackElement::Operator(Expressions::Add));
        calc.push(StackElement::Operand(3));
        assert_eq!(3, calc.stack.len());
    }
}
