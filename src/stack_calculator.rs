#[derive(Debug)]
pub enum Expressions {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

#[derive(Debug)]
pub struct StackCalculator {
    numbers: Vec<i32>,
    expressions: Vec<Expressions>,
}

impl StackCalculator {
    pub fn new() -> Self {
        StackCalculator {
            numbers: Vec::new(),
            expressions: Vec::new(),
        }
    }

    pub fn push_number(&mut self, num: i32) {
        self.numbers.push(num);
    }

    pub fn push_expression(&mut self, expression: Expressions) {
        self.expressions.push(expression)
    }

    pub fn evaluate(&mut self) -> Option<i32> {
        println!("Debug output: {:?}", self);

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::stack_calculator::{Expressions, StackCalculator};

    #[test]
    fn should_add_number_to_stack() {
        let mut stack = StackCalculator::new();
        stack.push_number(5);
        assert_eq!(1, stack.numbers.len());
    }

    #[test]
    fn should_add_expression_to_stack() {
        let mut stack = StackCalculator::new();
        stack.push_expression(Expressions::ADD);
        assert_eq!(1, stack.expressions.len());
    }
}
