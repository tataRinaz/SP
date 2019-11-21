use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub enum Node {
    Constant(f32),
    BinaryOperation(Operation, Box<Node>, Box<Node>),
    Variable(String),
    Assignment(String, Box<Node>),
    Function(String, Function),
    Call(String, Vec<Node>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Vec<Node>,
}

impl Function {
    fn call(
        &self,
        context: &mut Context,
        parameters: &[Node],
    ) -> Result<Option<f32>, Box<dyn std::error::Error>> {
        debug_assert_eq!(self.parameters.len(), parameters.len());
        let mut param_values = Vec::new();
        for (name, value) in self.parameters.iter().cloned().zip(parameters.iter()) {
            let value = value.evaluate(context)?.unwrap();
            param_values.push((name, value));
        }

        for (name, value) in param_values {
            context.variables.insert(name, value);
        }

        let mut value = None;
        for expression in self.body.iter() {
            value = expression.evaluate(context)?;
        }

        Ok(value)
    }
}

#[derive(Default, Clone)]
pub struct Context {
    variables: BTreeMap<String, f32>,
    functions: BTreeMap<String, Function>,
}

fn evaluate_binary_operation(
    operation: &Operation,
    left_node: &Box<Node>,
    right_node: &Box<Node>,
    context: &mut Context,
) -> Result<f32, Box<dyn std::error::Error>> {
    let left_value = left_node.evaluate(context)?.unwrap();
    let right_value = right_node.evaluate(context)?.unwrap();

    match operation {
        Operation::Plus => Ok(left_value + right_value),
        Operation::Minus => Ok(left_value - right_value),
        Operation::Divide => Ok(left_value / right_value),
        Operation::Multiply => Ok(left_value * right_value),
    }
}

impl Node {
    pub fn to_string(&self) -> String {
        match self {
            Node::Constant(number) => number.to_string(),
            Node::BinaryOperation(operation, left_node, right_node) => {
                left_node.to_string() + &operation.to_string() + &right_node.to_string()
            }
            Node::Variable(name) => name.clone(),
            Node::Assignment(name, value) => name.clone() + "=" + &value.to_string(),
            Node::Function(name, Function { parameters, body }) => {
                "fn ".to_string()
                    + &name
                    + "("
                    + &parameters.join(", ")
                    + ") {\n"
                    + &body
                        .iter()
                        .map(|expr| "  ".to_string() + &expr.to_string())
                        .collect::<Vec<String>>()
                        .join(";\n")
                    + "}"
            }
            Node::Call(name, params) => {
                name.clone()
                    + "("
                    + &params
                        .iter()
                        .map(|expr| expr.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + ")"
            }
        }
    }

    pub fn evaluate(
        &self,
        context: &mut Context,
    ) -> Result<Option<f32>, Box<dyn std::error::Error>> {
        match self {
            Node::Constant(number) => Ok(Some(*number)),
            Node::BinaryOperation(operation, left_node, right_node) => {
                evaluate_binary_operation(operation, left_node, right_node, context).map(Some)
            }
            Node::Variable(name) => {
                let variable = context.variables.get(name);
                match variable {
                    Some(value) => Ok(Some(*value)),
                    None => Err(format!("{} is not defined", name).into()),
                }
            }
            Node::Assignment(name, value) => {
                let value = value.evaluate(context)?;
                context.variables.insert(name.clone(), value.unwrap());
                Ok(None)
            }
            Node::Function(name, function) => {
                context.functions.insert(name.clone(), function.clone());
                Ok(None)
            }
            Node::Call(name, parameters) => {
                let function = context.functions.get(name);
                match function {
                    Some(function) => {
                        let mut context = context.clone();
                        if function.parameters.len() != parameters.len() {
                            return Err(format!(
                                "{} function takes {} params provided {}",
                                name,
                                function.parameters.len(),
                                parameters.len()
                            )
                            .into());
                        }
                        function.call(&mut context, parameters)
                    }
                    None => Err(format!("{} function is not defined", name).into()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Context, Node, Operation};
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
        let mut context = Context::default();
        assert_eq!(operation.evaluate(&mut context).unwrap(), Some(3.0));
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

        let mut context = Context::default();
        assert_eq!(minus.evaluate(&mut context).unwrap(), Some(-9.0));
        assert_eq!(minus.to_string(), "1+2-3*4");
    }
}
