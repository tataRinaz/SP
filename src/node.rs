use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub enum Operation {
    Plus,
    Minus,
    Divide,
    Multiply,
}

impl Operation {
    fn to_char(&self) -> char {
        match self {
            Operation::Plus => '+',
            Operation::Minus => '-',
            Operation::Divide => '/',
            Operation::Multiply => '*',
        }
    }

    pub fn from_char(c: char) -> Result<Operation, ()> {
        match c {
            '+' => Ok(Operation::Plus),
            '-' => Ok(Operation::Minus),
            '/' => Ok(Operation::Divide),
            '*' => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug)]
pub enum Node {
    Constant(f32),
    BinaryOperation(Operation, Box<Node>, Box<Node>),
}

struct Context {
    variables: BTreeMap<String, Node>,
    functions: BTreeMap<String, Node>,
}

fn evaluate_binary_operation(
    operation: &Operation,
    left_node: &Box<Node>,
    right_node: &Box<Node>,
) -> f32 {
    let left_value = left_node.evaluate();
    let right_value = right_node.evaluate();

    match operation {
        Operation::Plus => left_value + right_value,
        Operation::Minus => left_value - right_value,
        Operation::Divide => left_value / right_value,
        Operation::Multiply => left_value * right_value,
    }
}

impl Node {
    pub fn to_string(&self) -> String {
        match self {
            Node::Constant(number) => number.to_string(),
            Node::BinaryOperation(operation, left_node, right_node) => {
                left_node.to_string() + &operation.to_string() + &right_node.to_string()
            }
        }
    }

    pub fn evaluate(&self) -> f32 {
        match self {
            Node::Constant(number) => *number,
            Node::BinaryOperation(operation, left_node, right_node) => {
                evaluate_binary_operation(operation, left_node, right_node)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::node::Node;
    use crate::node::Operation;
    use Operation::*;

    fn num(num: f32) -> Node {
        Node::Constant(num)
    }

    fn bin(oper: Operation, left: Node, right: Node) -> Node {
        Node::BinaryOperation(oper, Box::new(left), Box::new(right))
    }
    #[test]
    fn basic_tree() {
        //  +
        // / \
        //1   2
        let operation = bin(Plus, num(1.0), num(2.0));
        assert_eq!(operation.evaluate(), 3.0);
        assert_eq!(operation.to_string(), "1+2");
    }

    #[test]
    fn two_operations() {
        //      -
        //   /     \
        //  +       *
        // / \     / \
        //1   2   3   4
        let left_oper = bin(Plus, num(1.0), num(2.0));
        let right_oper = bin(Multiply, num(3.0), num(4.0));
        let minus = bin(Minus, left_oper, right_oper);

        assert_eq!(minus.evaluate(), -9.0);
        assert_eq!(minus.to_string(), "1+2-3*4");
    }
}
