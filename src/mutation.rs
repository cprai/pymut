use rustpython_parser::ast;

pub enum MutationType {
    BinaryOperatorReplacement {new_operator: ast::Operator},
    ComparisonOperatorReplacement {new_operator: ast::Comparison, which: usize},
    NumberConstantReplacement {new_constant: ast::Number},
}

pub trait Mutation {
    fn mutate(&mut self, mutation_type: MutationType);
}

impl Mutation for ast::Expression {
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