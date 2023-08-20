#[derive(Debug)]
pub enum StackElement {
    Number(i32),
    Operator(Expressions),
}

#[derive(Debug)]
pub enum Expressions {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
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
}

#[cfg(test)]
mod tests {
    use crate::stack_calculator::{Expressions, StackCalculator, StackElement};

    #[test]
    fn should_add_number_to_stack() {
        let mut calc = StackCalculator::new();
        calc.push(StackElement::Number(5));
        assert_eq!(1, calc.stack.len());
    }

    #[test]
    fn should_add_expression_to_stack() {
        let mut calc = StackCalculator::new();
        calc.push(StackElement::Number(5));
        calc.push(StackElement::Operator(Expressions::ADD));
        calc.push(StackElement::Number(3));
        assert_eq!(3, calc.stack.len());
    }
}
