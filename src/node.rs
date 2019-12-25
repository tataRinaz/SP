use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Plus,
    Minus,
    Divide,
    Multiply,
    Less,
    More,
    Equal,
    NotEqual,
    Or,
    And,
}

impl Operation {
    pub fn from_string(c: &str) -> Result<Operation, ()> {
        match c {
            "+" => Ok(Operation::Plus),
            "-" => Ok(Operation::Minus),
            "/" => Ok(Operation::Divide),
            "*" => Ok(Operation::Multiply),
            "<" => Ok(Operation::Less),
            ">" => Ok(Operation::More),
            "==" => Ok(Operation::Equal),
            "!=" => Ok(Operation::NotEqual),
            "||" => Ok(Operation::Or),
            "&&" => Ok(Operation::And),
            _ => Err(()),
        }
    }

    pub fn is_arithmetic(&self) -> bool {
        match self {
            Operation::Plus | Operation::Minus | Operation::Divide | Operation::Multiply => true,
            _ => false,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String;
        match self {
            Operation::Plus => result = "+".to_string(),
            Operation::Minus => result = "-".to_string(),
            Operation::Divide => result = "/".to_string(),
            Operation::Multiply => result = "*".to_string(),
            Operation::Less => result = "<".to_string(),
            Operation::More => result = ">".to_string(),
            Operation::Equal => result = "==".to_string(),
            Operation::NotEqual => result = "!=".to_string(),
            Operation::Or => result = "||".to_string(),
            Operation::And => result = "&&".to_string(),
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    None,
    Bool(bool),
    Number(f32),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::None => "None".to_string(),
            Value::Bool(boolean) => boolean.to_string(),
            Value::Number(number) => number.to_string(),
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn to_number(&self) -> Option<f32> {
        match self {
            Value::Number(number) => Some(*number),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(boolean) => Some(*boolean),
            _ => None,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Value::None => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Constant(Value),
    BinaryOperation(Operation, Box<Node>, Box<Node>),
    Variable(String),
    Block(Vec<Node>),
    Assignment(String, Box<Node>),
    Function(String, Function),
    Call(String, Vec<Node>),
    IfElse(
        Box<Node>,         /* condition */
        Box<Node>,         /* if true */
        Option<Box<Node>>, /* if false */
    ),
    While(Box<Node> /* condition */, Box<Node> /* body */),
    For(
        Box<Node>, /* init */
        Box<Node>, /* condition */
        Box<Node>, /* body */
        Box<Node>, /* step */
    ),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Box<Node>,
}

impl Function {
    fn call(
        &self,
        context: &mut Context,
        parameters: &[Node],
    ) -> Result<Value, Box<dyn std::error::Error>> {
        debug_assert_eq!(self.parameters.len(), parameters.len());
        let mut param_values = Vec::new();
        for (name, value) in self.parameters.iter().cloned().zip(parameters.iter()) {
            let value = value.evaluate(context);
            param_values.push((name, value));
        }

        for (name, value) in param_values {
            context.variables.insert(name, value.unwrap());
        }

        let value = self.body.evaluate(context)?;
        Ok(value)
    }
}

#[derive(Default, Clone)]
pub struct Context {
    variables: BTreeMap<String, Value>,
    functions: BTreeMap<String, Function>,
}

fn evaluate_binary_operation(
    operation: &Operation,
    left_value: f32,
    right_value: f32,
) -> Result<Value, Box<dyn std::error::Error>> {
    match operation {
        Operation::Plus => Ok(Value::Number(left_value + right_value)),
        Operation::Minus => Ok(Value::Number(left_value - right_value)),
        Operation::Divide => Ok(Value::Number(left_value / right_value)),
        Operation::Multiply => Ok(Value::Number(left_value * right_value)),
        _ => Err(format!("Logical operation in arithmetical expression").into()),
    }
}

fn evaluate_logical_operation(
    operation: &Operation,
    left_value: Value,
    right_value: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    match left_value {
        Value::Number(left) => {
            let right = right_value.to_number().unwrap();
            match operation {
                Operation::Less => Ok(Value::Bool(left < right)),
                Operation::More => Ok(Value::Bool(left > right)),
                Operation::Equal => Ok(Value::Bool(left == right)),
                Operation::NotEqual => Ok(Value::Bool(left != right)),
                _ => Err(format!("Arithemtical operation in logical expression").into()),
            }
        }
        Value::Bool(left) => {
            let right = right_value.to_bool().unwrap();
            match operation {
                Operation::Equal => Ok(Value::Bool(left == right)),
                Operation::NotEqual => Ok(Value::Bool(left != right)),
                Operation::Or => Ok(Value::Bool(left || right)),
                Operation::And => Ok(Value::Bool(left && right)),
                _ => Err(format!("Arithemtical operation in logical expression").into()),
            }
        }
        _ => Err(format!("None as operand in logical operation").into()),
    }
}

fn evaluate_condition(
    condition: &Box<Node>,
    context: &mut Context,
) -> Result<bool, Box<dyn std::error::Error>> {
    let cond_result = condition.evaluate(context)?;
    Ok(
        cond_result.is_bool() && cond_result.to_bool().unwrap() == true
            || cond_result.is_number() && cond_result.to_number().unwrap() == 0.0,
    )
}

fn evaluate_operation(
    operation: &Operation,
    left_node: &Node,
    right_node: &Node,
    context: &mut Context,
) -> Result<Value, Box<dyn std::error::Error>> {
    let left_value = left_node.evaluate(context)?;
    let right_value = right_node.evaluate(context)?;

    if operation.is_arithmetic() {
        if !left_value.is_number() || !right_value.is_number() {
            return Err(format!("One of operands in arithmetic expression is not number").into());
        }
        return evaluate_binary_operation(
            operation,
            left_value.to_number().unwrap(),
            right_value.to_number().unwrap(),
        );
    } else {
        if left_value.is_none() || right_value.is_none() {
            return Err(format!("None value in binary expression").into());
        }

        if left_value.is_bool() && right_value.is_number()
            || left_value.is_number() && right_value.is_bool()
        {
            return Err(format!("Operands have different types in expression").into());
        }

        return evaluate_logical_operation(operation, left_value, right_value);
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
            Node::Block(body) => body
                .iter()
                .map(|expr| "  ".to_string() + &expr.to_string())
                .collect::<Vec<String>>()
                .join(";\n"),
            Node::Function(name, Function { parameters, body }) => {
                "fn ".to_string()
                    + &name
                    + "("
                    + &parameters.join(", ")
                    + ") {\n"
                    + &body.to_string()
                    + "}\n"
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
            Node::IfElse(condition, if_body, else_body) => {
                let result = "if ".to_string()
                    + &condition.to_string()
                    + " {\n"
                    + &if_body.to_string()
                    + "}\n";
                if else_body.is_some() {
                    let body = else_body.as_ref().unwrap();
                    let result = result + "else {\n" + &body.to_string() + "}\n";
                    return result;
                }
                result
            }
            Node::While(condition, body) => {
                "while ".to_string() + &condition.to_string() + " {\n" + &body.to_string() + "}\n"
            }
            Node::For(init, condition, body, step) => {
                "for ".to_string()
                    + &init.to_string()
                    + "; "
                    + &condition.to_string()
                    + "; "
                    + &step.to_string()
                    + " {\n"
                    + &body.to_string()
                    + "}\n"
            }
        }
    }

    pub fn evaluate(&self, context: &mut Context) -> Result<Value, Box<dyn std::error::Error>> {
        match self {
            Node::Constant(number) => Ok(*number),
            Node::BinaryOperation(operation, left_node, right_node) => {
                evaluate_operation(operation, left_node, right_node, context)
            }
            Node::Variable(name) => {
                let variable = context.variables.get(name);
                match variable {
                    Some(value) => Ok(*value),
                    None => Err(format!("{} is not defined", name).into()),
                }
            }
            Node::Assignment(name, value) => {
                let value = value.evaluate(context)?;
                context.variables.insert(name.clone(), value);
                Ok(Value::None)
            }
            Node::Block(body) => {
                let mut value = Value::None;
                for expression in body.iter() {
                    value = expression.evaluate(context)?;
                }
                Ok(value)
            }
            Node::Function(name, function) => {
                context.functions.insert(name.clone(), function.clone());
                Ok(Value::None)
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
            Node::IfElse(condition, if_body, else_body) => {
                let cond = evaluate_condition(condition, context)?;
                if cond {
                    if_body.evaluate(context)
                } else if else_body.is_some() {
                    else_body.as_ref().unwrap().evaluate(context)
                } else {
                    Ok(Value::None)
                }
            }
            Node::While(condition, body) => {
                while evaluate_condition(condition, context)? {
                    body.evaluate(context)?;
                }
                Ok(Value::None)
            }
            Node::For(init, condition, body, step) => {
                init.evaluate(context);
                while evaluate_condition(condition, context)? {
                    body.evaluate(context)?;
                    step.evaluate(context)?;
                }
                Ok(Value::None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Context, Node, Operation, Value};
    use Operation::*;

    fn num(num: f32) -> Node {
        Node::Constant(Value::Number(num))
    }

    fn logic(boolean: bool) -> Node {
        Node::Constant(Value::Bool(boolean))
    }

    fn bin(oper: Operation, left: Node, right: Node) -> Node {
        Node::BinaryOperation(oper, Box::new(left), Box::new(right))
    }

    fn ifelse(condition: Node, if_expr: Node, else_expr: Option<Node>) -> Node {
        if else_expr.is_none() {
            Node::IfElse(Box::new(condition), Box::new(if_expr), None)
        } else {
            Node::IfElse(
                Box::new(condition),
                Box::new(if_expr),
                Some(Box::new(else_expr.unwrap())),
            )
        }
    }

    fn block(body: Vec<Node>) -> Node {
        Node::Block(body)
    }

    #[test]
    fn basic_tree() {
        //  +
        // / \
        //1   2
        let operation = bin(Plus, num(1.0), num(2.0));
        let mut context = Context::default();
        let value = operation.evaluate(&mut context).unwrap();
        assert_eq!(value.to_number().unwrap(), 3.0);
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
        assert_eq!(
            minus.evaluate(&mut context).unwrap().to_number().unwrap(),
            -9.0
        );
        assert_eq!(minus.to_string(), "1+2-3*4");
    }

    #[test]
    fn simple_logical() {
        let operation = bin(Less, num(3.0), num(4.0));
        let mut context = Context::default();
        let value = operation.evaluate(&mut context).unwrap();
        assert_eq!(value.to_bool().unwrap(), true);
        assert_eq!(operation.to_string(), "3<4");
    }

    #[test]
    fn different_types() {
        let left_oper = bin(More, num(4.0), num(3.0));
        let operation = bin(More, left_oper, num(2.0));
        let mut context = Context::default();
        let value = operation.evaluate(&mut context);
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Operands have different types in expression"
        )
    }

    #[test]
    fn simple_if_expression() {
        let mut context = Context::default();
        let body = [bin(Plus, num(1.0), num(2.0))];
        let condition = bin(Less, num(3.0), num(4.0));
        let if_else = ifelse(condition, block(body.to_vec()), None);
        let value = if_else.evaluate(&mut context).unwrap();
        assert_eq!(value.to_number().unwrap(), 3.0)
    }
    #[test]
    fn simple_else_expression() {
        let mut context = Context::default();
        let body_if = [bin(Plus, num(1.0), num(2.0))];
        let body_else = [bin(Plus, num(3.0), num(4.0))];
        let condition = bin(More, num(3.0), num(4.0));
        let if_else = ifelse(
            condition,
            block(body_if.to_vec()),
            Some(block(body_else.to_vec())),
        );
        let value = if_else.evaluate(&mut context).unwrap();
        assert_eq!(value.to_number().unwrap(), 7.0)
    }
}
