use crate::core::{Parser, predicate, zero_or_more, one_or_more};

use std::fmt::{Formatter, Display, Error};
use std::fmt;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Formula {
    Id(String),
    Map(String, Box<Expression>)
}

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Formula::Id(name) => write!(f, "{}", name),
            Formula::Map(_, _) => write!(f, "{}", ""),
        }
    }
}



#[derive(Clone, Debug, PartialEq, Eq)]
pub enum  Expression {
    Num(String),
    Variable(String),
    Map(Box<Formula>),
    Binary( Box<Expression>, Operator, Box<Expression>),
    Assign(Box<Expression>, Box<Expression>)
}


pub struct  Code {
    expressions: Vec<Expression>
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Num(a) => write!(f,"Number: {}",a),
            Expression::Variable(a) => write!(f,"Number: {}",a),
            Expression::Map(_) => write!(f,"Number: {}", ""),
            Expression::Binary(_, _, _) => write!(f,"Number: {}",""),
            Expression::Assign(_, _) => write!(f,"Number: {}",""),
        }
    }
}

pub fn any_char<'a> () -> impl Parser<'a, char>
{
    move |input: &'a str| {
        match input.chars().next() {
            Some(value) => Ok((&input[value.len_utf8()..], value)),
            _ => Err(input)
        }
    }
}


pub fn literal<'a>(id: &'a str) -> impl Parser<'a, Expression > {

    move |input: &'a str| {
        match input.get(0..id.len()) {
            Some(value) if value == id => Ok( (&input[id.len()..], Expression::Variable(value.to_string()) ) ),
            _ => Err(input)
        }
    }
}



pub fn whitespaces<'a>() -> impl Parser<'a, () > {

    |input| {

        let spaces = predicate(any_char(), |c| c.is_whitespace());

        let parser = one_or_more(spaces);
        if let Ok((next, _token)) = parser.parse(input) {
            return Ok((next, ()));
        }

        Err(input)
    }
}

pub fn identifier<'a>() -> impl Parser<'a, Formula> {
    move |input| {
        let alpha = predicate(any_char(), |c| c.is_alphabetic());
        let alphanumeric = zero_or_more(predicate(any_char(), |c| c.is_alphanumeric()));

        if let Ok((next, result)) = alpha.parse(input) {
            if let Ok((last, mut results)) = alphanumeric.parse(next) {
                results.insert(0, result);
                return Ok((last, Formula::Id(results.into_iter().collect())))
            }

            return Err(next)
        } else {
            Err(input)
        }
    }
}

/*
pub fn formula_parser<'a>() -> impl Parser<'a, Formula> {
    move |input| {




    }


}

pub fn math_lang<'a>() -> impl Parser<'a, Code >

{

    |input| {





    }

}
*/
