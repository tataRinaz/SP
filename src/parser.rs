use crate::node::{Function, Node, Operation, Value};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while1};
use nom::character::is_alphabetic;
use nom::combinator::{map, map_res, opt};
use nom::error_position;
use nom::multi::fold_many0;
use nom::number::complete::float;
use nom::sequence::tuple;
use nom::IResult;

fn identifier(input: &[u8]) -> IResult<&[u8], String> {
    map(take_while1(is_alphabetic), |variable: &[u8]| {
        String::from_utf8(variable.to_vec()).unwrap()
    })(input)
}

fn variable(input: &[u8]) -> IResult<&[u8], Node> {
    map(identifier, |variable: String| Node::Variable(variable))(input)
}

fn number(input: &[u8]) -> IResult<&[u8], Node> {
    map(float, |num: f32| Node::Constant(Value::Number(num)))(input)
}

fn operation(input: &[u8]) -> IResult<&[u8], Operation> {
    map(
        alt((
            tag("||"),
            tag("&&"),
            tag("=="),
            tag("!="),
            tag("+"),
            tag("-"),
            tag("/"),
            tag("*"),
            tag(">"),
            tag("<"),
        )),
        |c: &[u8]| Operation::from_string(std::str::from_utf8(c).unwrap()).unwrap(),
    )(input)
}

fn plus_minus_oper(input: &[u8]) -> IResult<&[u8], Operation> {
    let (input, operation) = operation(input)?;
    if operation == Operation::Plus || operation == Operation::Minus {
        Ok((&input, operation))
    } else {
        Err(nom::Err::Error(error_position!(
            input,
            nom::error::ErrorKind::MapRes
        )))
    }
}

fn div_multi_oper(input: &[u8]) -> IResult<&[u8], Operation> {
    let (input, operation) = operation(input)?;
    if (operation == Operation::Multiply) || (operation == Operation::Divide) {
        Ok((&input, operation))
    } else {
        Err(nom::Err::Error(error_position!(
            input,
            nom::error::ErrorKind::MapRes
        )))
    }
}

fn logic_oper(input: &[u8]) -> IResult<&[u8], Operation> {
    let (input, operation) = operation(input)?;
    if (operation == Operation::NotEqual)
        || (operation == Operation::Equal)
        || (operation == Operation::Or)
        || (operation == Operation::And)
        || (operation == Operation::Less)
        || (operation == Operation::More)
    {
        Ok((&input, operation))
    } else {
        Err(nom::Err::Error(error_position!(
            input,
            nom::error::ErrorKind::MapRes
        )))
    }
}

fn brackets_expression(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = tag("(")(input)?;
    let (input, expr) = map(tuple((space, expression, space)), |(_, expr, _)| expr)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, expr))
}

fn unary_minus(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("-")(input)
}

fn factor(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = space(input)?;
    let (input, minus) = opt(unary_minus)(input)?;
    let (input, _) = space(input)?;
    let (input, expression) = alt((number, call, variable, brackets_expression))(input)?;

    if minus.is_some() {
        Ok((
            input,
            Node::BinaryOperation(
                Operation::Minus,
                Box::new(Node::Constant(Value::Number(0.0))),
                Box::new(expression),
            ),
        ))
    } else {
        Ok((input, expression))
    }
}

fn logic(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, left) = factor(input)?;
    let (input, _) = space(input)?;
    if let Ok((input, operation)) = logic_oper(input) {
        let (input, right) = logic(input)?;
        Ok((
            input,
            Node::BinaryOperation(operation, Box::new(left), Box::new(right)),
        ))
    } else {
        Ok((input, left))
    }
}

fn term(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, left) = logic(input)?;
    let (input, _) = space(input)?;
    if let Ok((input, operation)) = div_multi_oper(input) {
        let (input, right) = term(input)?;
        Ok((
            input,
            Node::BinaryOperation(operation, Box::new(left), Box::new(right)),
        ))
    } else {
        Ok((input, left))
    }
}

fn expression(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = space(input)?;
    let (input, left) = term(input)?;
    let (input, _) = space(input)?;
    if let Ok((input, operation)) = plus_minus_oper(input) {
        let (input, right) = expression(input)?;
        Ok((
            input,
            Node::BinaryOperation(operation, Box::new(left), Box::new(right)),
        ))
    } else {
        Ok((input, left))
    }
}

fn space(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b' ')(input)
}

fn function(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = space(input)?;
    let (input, _) = tag("fn")(input)?;
    let (input, _) = space(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = space(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = space(input)?;

    let (input, parameters) = if let Ok((input, param)) = identifier(input) {
        fold_many0(
            tuple((space, tag(","), space, identifier)),
            vec![param],
            |mut params, (_, _, _, param_name)| {
                params.push(param_name);
                params
            },
        )(input)?
    } else {
        (input, Vec::new())
    };
    let (input, _) = space(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = space(input)?;

    let (input, body) = body(input)?;
    let (input, _) = space(input)?;
    let (input, _) = tag("}")(input)?;
    let boxed_body = Box::new(Node::Block(body));
    Ok((
        input,
        Node::Function(
            name,
            Function {
                parameters,
                body: boxed_body,
            },
        ),
    ))
}

fn body(input: &[u8]) -> IResult<&[u8], Vec<Node>> {
    let (input, _) = space(input)?;
    let (input, _) = tag("{")(input)?;
    fold_many0(
        tuple((space, statement, space, tag(";"))),
        Vec::new(),
        |mut body, (_, statement, _, _)| {
            body.push(statement);
            body
        },
    )(input)
}

fn call(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = space(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = space(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = space(input)?;

    let (input, parameters) = if let Ok((input, param)) = expression(input) {
        fold_many0(
            tuple((space, tag(","), space, expression)),
            vec![param],
            |mut params, (_, _, _, param)| {
                params.push(param);
                params
            },
        )(input)?
    } else {
        (input, Vec::new())
    };

    let (input, _) = space(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node::Call(name, parameters)))
}

fn else_block(input: &[u8]) -> IResult<&[u8], Option<Box<Node>>> {
    let (input, _) = space(input)?;
    let (input, opt_else) = opt(tag("else"))(input)?;
    if opt_else.is_none() {
        Ok((input, None))
    } else {
        let (input, _) = space(input)?;
        let (input, body) = body(input)?;
        let boxed_body = Box::new(Node::Block(body));
        let (input, _) = space(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, Some(boxed_body)))
    }
}

fn if_else(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = space(input)?;
    let (input, _) = tag("if")(input)?;
    let (input, _) = space(input)?;
    let (input, condition) = expression(input)?;
    let (input, _) = space(input)?;

    let (input, if_body) = body(input)?;
    let (input, _) = space(input)?;
    let (input, _) = tag("}")(input)?;
    let boxed_body = Box::new(Node::Block(if_body));
    let (input, else_body) = else_block(input)?;

    Ok((
        input,
        Node::IfElse(Box::new(condition), boxed_body, else_body),
    ))
}

fn while_ident(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, _) = space(input)?;
    let (input, _) = tag("while")(input)?;
    let (input, _) = space(input)?;
    let (input, condition) = expression(input)?;
    let (input, _) = space(input)?;

    let (input, body) = body(input)?;
    let (input, _) = space(input)?;
    let (input, _) = tag("}")(input)?;
    let boxed_body = Box::new(Node::Block(body));

    Ok((
        input,
        Node::While(Box::new(condition), boxed_body)
    ))
}
// Backus-Naur Form of math expression
//
// Statement ::=  Function| While| IfElse | Assignment | Expr
//
// Function ::= "fn" Var '(' [Var (',' Var)*]')' Body
// Body ::= '{' (Statement ';')* '}'
// Call ::= Var '(' [Expr (',' Expr)*]')'
//
// IfElse ::= "if" Expr Body ["else" Body]
// While  ::= "while" Expr Body
//
// Assignment ::= Var '=' Expr
// Var ::= Char+
//
// Expr ::= Term ('+' Term | '-' Term)*
// Term ::= Logic ('*' Logic | '/' Logic)*
// Logic ::= Factor ('>' Factor | '<' Factor | '==' Factor | '!=' Factor | '||' Factor | '&&' Factor)*
// Factor ::= ['-'] (Number | Call | '(' Expr ')')
//
// Number ::= Digit+

pub fn statement(input: &[u8]) -> IResult<&[u8], Node> {
    alt((function, while_ident, if_else, assignment, expression))(input)
}

fn assignment(input: &[u8]) -> IResult<&[u8], Node> {
    map(
        tuple((space, identifier, space, tag("="), space, expression)),
        |(_, variable, _, _, _, expression)| Node::Assignment(variable, Box::new(expression)),
    )(input)
}

#[cfg(test)]

mod tests {
    use crate::node::{Context, Value};
    use crate::parser::statement;
    fn eval(e: &str) -> Result<f32, Box<dyn std::error::Error>> {
        let (_, parsed) = statement(e.as_bytes()).map_err(|err| format!("{:?}", err))?;

        let mut context = Context::default();
        let value = parsed.evaluate(&mut context).unwrap();
        Ok(value.to_number().unwrap())
    }
    #[test]
    fn basic_expression() {
        assert_eq!(6.0, eval("1+2+3").unwrap());
    }

    #[test]
    fn basic_expression_with_spaces() {
        assert_eq!(6.0, eval("1 + 2 + 3").unwrap());
    }
    #[test]
    fn expression_with_multiply() {
        assert_eq!(8.0, eval("1+2*3.5").unwrap());
    }

    #[test]
    fn expression_with_brackets() {
        assert_eq!(5.0, eval("3+4*(6.5-6)").unwrap());
    }
}
