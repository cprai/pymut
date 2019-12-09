use serde::{Serialize, Deserialize};

use rustpython_parser::ast::Operator;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Operator")]
pub enum OperatorSerde {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

//use rustpython_parser::ast::BooleanOperator;
//
//#[derive(Serialize, Deserialize)]
//#[serde(remote = "BooleanOperator")]
//pub enum BooleanOperatorSerde {
//    And,
//    Or,
//}
//
//use rustpython_parser::ast::UnaryOperator;
//
//#[derive(Serialize, Deserialize)]
//#[serde(remote = "UnaryOperator")]
//pub enum UnaryOperatorSerde {
//    Pos,
//    Neg,
//    Not,
//    Inv,
//}

use rustpython_parser::ast::Comparison;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Comparison")]
pub enum ComparisonSerde {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    In,
    NotIn,
    Is,
    IsNot,
}

use num_bigint::BigInt;
use rustpython_parser::ast::Number;

#[derive(Serialize, Deserialize)]
#[serde(remote = "Number")]
pub enum NumberSerde {
    Integer { value: BigInt },
    Float { value: f64 },
    Complex { real: f64, imag: f64 },
}