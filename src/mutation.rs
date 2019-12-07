use rustpython_parser::ast;
use serde::{Serialize, Deserialize};

use crate::serde_compatibility::OperatorSerde;
use crate::serde_compatibility::ComparisonSerde;
use crate::serde_compatibility::NumberSerde;

#[derive(Serialize, Deserialize, Clone)]
pub struct Mutation {
    pub traversal_location: u64,
    pub mutation_type: MutationType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MutationType {
    #[serde(with = "OperatorSerde")]
    BinaryOperatorReplacement {new_operator: ast::Operator},

    #[serde(with = "ComparisonSerde")]
    ComparisonOperatorReplacement {new_operator: ast::Comparison},

    #[serde(with = "NumberSerde")]
    NumberConstantReplacement {new_constant: ast::Number},
}

pub trait Mutate {
    fn mutate(&mut self, mutation_type: MutationType);
}

impl Mutate for ast::Expression {
    fn mutate(&mut self, mutation_type: MutationType) {
        match mutation_type {

            MutationType::BinaryOperatorReplacement {new_operator} => {
                match &mut self.node {
                    ast::ExpressionType::Binop {a: _, op, b: _} => {
                        *op = new_operator;
                    },

                    _ => unreachable!(),
                }
            },

            MutationType::ComparisonOperatorReplacement {new_operator} => {
                match &mut self.node {
                    ast::ExpressionType::Compare {vals: _, ops} => {
                        ops[0] = new_operator;
                    },

                    _ => unreachable!(),
                }
            },

            _ => unreachable!(),

        }
    }
}