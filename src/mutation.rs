use rustpython_parser::ast;

pub trait BinaryOpToMultiplicationMutation {
    fn mutate(&mut self);
}

impl BinaryOpToMultiplicationMutation for ast::Expression {
    fn mutate(&mut self) {
        match &mut self.node {
            ast::ExpressionType::Binop {a: _, op, b: _} => {
                *op = ast::Operator::Mult;
            }

            _ => unreachable!()
        }
    }
}

//impl BinaryOpToMultiplicationMutation for ast::Expression {
//    fn mutate(&mut self) {
//        match &mut self.node {
//            ast::ExpressionType::Binop {a: _, op, b: _} => {
//                *op = ast::Operator::Mult;
//            }
//
//            _ => unreachable!()
//        }
//    }
//}