use rustpython_parser::ast;
use serde::{Serialize, Deserialize};

use crate::traversal::Visitor;

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

            MutationType::NumberConstantReplacement {new_constant} => {
                match &mut self.node {
                    ast::ExpressionType::Number {value} => {
                        *value = new_constant;
                    },

                    _ => unreachable!(),
                }
            },

            //_ => unreachable!(),

        }
    }
}

pub fn explore_mutations(program: &mut ast::Program) -> Vec<Mutation> {
    let mut mutations: Vec<Mutation> = Vec::new();
    let mut i: u64 = 0;

    program.visit(&mut |expr| {
        i += 1;

        match &expr.node {

            ast::ExpressionType::Binop {..} => {
                {
                    let mutation = MutationType::BinaryOperatorReplacement{new_operator: ast::Operator::Mult};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
                {
                    let mutation = MutationType::BinaryOperatorReplacement{new_operator: ast::Operator::Sub};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
                {
                    let mutation = MutationType::BinaryOperatorReplacement{new_operator: ast::Operator::Pow};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
            },

            ast::ExpressionType::Number {value} => {
                fn plus_one(number: &ast::Number) -> ast::Number {
                    match &number {
                        ast::Number::Integer {value} => ast::Number::Integer{value: value + 1},
                        ast::Number::Float {value} => ast::Number::Float{value: value + 1.0},
                        ast::Number::Complex {real, imag} => ast::Number::Complex{real: real + 1.0, imag: *imag},
                    }
                }
                fn minus_one(number: &ast::Number) -> ast::Number {
                    match &number {
                        ast::Number::Integer {value} => ast::Number::Integer{value: value - 1},
                        ast::Number::Float {value} => ast::Number::Float{value: value - 1.0},
                        ast::Number::Complex {real, imag} => ast::Number::Complex{real: real - 1.0, imag: *imag},
                    }
                }
                fn from_i64(value: i64) -> ast::Number {
                    ast::Number::Integer{value: num_bigint::BigInt::from(value)}
                }

                {
                    let mutation = MutationType::NumberConstantReplacement{new_constant: plus_one(&value)};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
                {
                    let mutation = MutationType::NumberConstantReplacement{new_constant: minus_one(&value)};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
                {
                    let mutation = MutationType::NumberConstantReplacement{new_constant: from_i64(0)};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
                {
                    let mutation = MutationType::NumberConstantReplacement{new_constant: from_i64(1)};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
                {
                    let mutation = MutationType::NumberConstantReplacement{new_constant: from_i64(-1)};
                    mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                }
            },

            _ => (),
        }
    });

    return mutations;
}

pub fn apply_mutation(program: &mut ast::Program, mutation: Mutation) {
    let mut i: u64 = 0;

    program.visit(&mut |expr| {
        i += 1;

        if i == mutation.traversal_location {
            expr.mutate(mutation.mutation_type.clone());
        }
    });
}