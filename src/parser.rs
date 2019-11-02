use crate::node::{Node, Operation};
use nom::branch::alt;
use nom::bytes::complete::{take, take_while};
use nom::combinator::{map, map_res};
use nom::multi::fold_many0;
use nom::number::complete::float;
use nom::sequence::tuple;
use nom::IResult;

fn number(input: &[u8]) -> IResult<&[u8], Node> {
    map(float, Node::Constant)(input)
}

fn operation(input: &[u8]) -> IResult<&[u8], Operation> {
    map_res(take(1usize), |c: &[u8]| Operation::from_char(c[0] as char))(input)
}

fn space(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b' ')(input)
}

fn binary_operation(input: &[u8]) -> IResult<&[u8], Node> {
    map(
        tuple((number, space, operation, space, number)),
        |(left, _, operation, _, right)| {
            Node::BinaryOperation(operation, Box::new(left), Box::new(right))
        },
    )(input)
}

fn operations(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, left) = binary_operation(input)?;
    if let Ok((input, operation)) = operation(input) {
        let (input, right) = linear_expression(input)?;
        Ok((
            input,
            Node::BinaryOperation(operation, Box::new(left), Box::new(right)),
        ))
    } else {
        Ok((input, left))
    }
}

pub fn linear_expression(input: &[u8]) -> IResult<&[u8], Node> {
    alt((operations, number))(input)
}

