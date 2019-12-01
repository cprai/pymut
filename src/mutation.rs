use rustpython_parser::ast;

pub struct Mutation {
    pub traversal_location: u64,
    pub mutation_type: MutationType,
}

#[derive(Clone)]
pub enum MutationType {
    BinaryOperatorReplacement {new_operator: ast::Operator},
    ComparisonOperatorReplacement {new_operator: ast::Comparison, which: usize},
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

            MutationType::ComparisonOperatorReplacement {new_operator, which} => {
                match &mut self.node {
                    ast::ExpressionType::Compare {vals: _, ops} => {
                        ops[which] = new_operator;
                    },

                    _ => unreachable!(),
                }
            },

            _ => unreachable!(),

        }
    }
}