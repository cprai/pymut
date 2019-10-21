use rustpython_parser::{ast, parser, location};



fn walk_expression(expression: &ast::Expression, parent_id: &String, graph: &mut Vec<String>) {
    let id: String = id_from_location("expression", &expression.location);
    insert_edge(&parent_id, &id, graph);
    match &expression.node {
        ast::ExpressionType::Identifier {name} => insert_node(&id, &name, graph),

        ast::ExpressionType::Number {value} => {
            let as_string: String = match value {
                ast::Number::Integer {value} => format!("{}", value),
                ast::Number::Float {value} => format!("{}", value),
                ast::Number::Complex {real, imag} => format!("{}+i{}", real, imag),
            };
            insert_node(&id, &as_string, graph)
        },

        ast::ExpressionType::BoolOp {op, values} => {
            let operator: &str = match op {
                ast::BooleanOperator::And => "and",
                ast::BooleanOperator::Or => "or",
            };

            insert_node(&id, operator, graph);
            for expression in values {
                walk_expression(&expression, &id, graph);
            }
        },

        ast::ExpressionType::Binop {a, op, b} => {
            let operator: &str = match op {
                ast::Operator::Add => "+",
                ast::Operator::Sub => "-",
                ast::Operator::Mult => "*",
                ast::Operator::MatMult => "@",
                ast::Operator::Div => "/",
                ast::Operator::Mod => "%",
                ast::Operator::Pow => "**",
                ast::Operator::LShift => "<<",
                ast::Operator::RShift => ">>",
                ast::Operator::BitOr => "|",
                ast::Operator::BitXor => "^",
                ast::Operator::BitAnd => "&",
                ast::Operator::FloorDiv => "//",
            };

            insert_node(&id, operator, graph);
            walk_expression(&*a, &id, graph);
            walk_expression(&*b, &id, graph);
        },

        ast::ExpressionType::Unop {op, a} => {
            let operator: &str = match op {
                ast::UnaryOperator::Pos => "+",
                ast::UnaryOperator::Neg => "-",
                ast::UnaryOperator::Not => "not",
                ast::UnaryOperator::Inv => "~",
            };

            insert_node(&id, operator, graph);
            walk_expression(&*a, &id, graph);
        },

        ast::ExpressionType::Compare {vals, ops} => {
            fn get_operator(operator: &ast::Comparison) -> &'static str {
                match operator {
                    ast::Comparison::Equal => "==",
                    ast::Comparison::NotEqual => "!=",
                    ast::Comparison::Less => "<",
                    ast::Comparison::LessOrEqual => "<=",
                    ast::Comparison::Greater => ">",
                    ast::Comparison::GreaterOrEqual => ">=",
                    ast::Comparison::In => "in",
                    ast::Comparison::NotIn => "not in",
                    ast::Comparison::Is => "is",
                    ast::Comparison::IsNot => "is not",
                }
            }

            if vals.len() == 2 && ops.len() == 1 {
                insert_node(&id, get_operator(&ops[0]), graph);
                walk_expression(&vals[0], &id, graph);
                walk_expression(&vals[1], &id, graph);
            }
            else {
                let vals_id: String = format!("{}_vals", &id);
                insert_node(&vals_id, "values", graph);

                for expression in vals {
                    walk_expression(&expression, &vals_id, graph);
                }

                let ops_id: String = format!("{}_ops", &id);
                insert_node(&ops_id, "operators", graph);

                let mut operator_id_counter: usize = 0;
                for comparison in ops {
                    let operator_id: String = format!("{}_ops_{}", &id, operator_id_counter);
                    insert_node(&operator_id, get_operator(&comparison), graph);
                    insert_edge(&ops_id, &operator_id, graph);

                    operator_id_counter += 1;
                }

                insert_node(&id, "multicomparison", graph);
                insert_edge(&id, &vals_id, graph);
                insert_edge(&id, &ops_id, graph);
            }
        },

        ast::ExpressionType::True => insert_node(&id, "True", graph),

        ast::ExpressionType::False => insert_node(&id, "False", graph),

        ast::ExpressionType::None => insert_node(&id, "None", graph),

        _ => unreachable!(),
    }
}

fn walk_suite(suite: &ast::Suite, parent_id: &String, graph: &mut Vec<String>) {
    for statement in suite {
        let id: String = id_from_location("statement", &statement.location);

        match &statement.node {
            ast::StatementType::Break => {
                insert_edge(&parent_id, &id, graph);
                insert_node(&id, "break", graph);
            },

            ast::StatementType::Continue => {
                insert_edge(&parent_id, &id, graph);
                insert_node(&id, "continue", graph);
            },

            ast::StatementType::Pass => {
                insert_edge(&parent_id, &id, graph);
                insert_node(&id, "pass", graph);
            },

            ast::StatementType::Assign {targets, value} => {
                if targets.len() == 1 {
                    insert_edge(&parent_id, &id, graph);
                    insert_node(&id, "=", graph);
                    walk_expression(&targets[0], &id, graph);
                    walk_expression(&value, &id, graph);
                }
                else {
                    unreachable!();
                }
            },

            ast::StatementType::Expression {expression} => {
                walk_expression(&expression, &id, graph);
            },

            //ast::StatementType::If {test, body, orelse} => {
            //    insert_node(id, "if", graph);
            //    walk_suite(body, id, graph);
            //    if orelse.is_some() {
            //        let else_id: &str = id + "100000000";
            //        insert_node(else_id, "else", graph);
            //        insert_edge(id, else_id, graph);
            //        walk_suite(orelse.unwrap(), else_id, graph);
            //    }
            //},

            ast::StatementType::While {test, body, orelse} => {
                let condition_id: String = format!("{}_condition", &id);
                let block_id: String = format!("{}_block", &id);

                insert_edge(&parent_id, &id, graph);
                insert_node(&id, "while", graph);

                //graph.push(format!("subgraph a {{"));
                //graph.push(format!("rankdir=LR;"));
                //graph.push(format!("rank=same;"));
                //graph.push(format!("{{rank=same;{},{}}}", &condition_id, &block_id));

                insert_edge(&id, &condition_id, graph);
                insert_node(&condition_id, "condition", graph);

                insert_edge(&id, &block_id, graph);
                insert_node(&block_id, "loopBody", graph);

                walk_expression(&test, &condition_id, graph);
                walk_suite(&body, &block_id, graph);

                //graph.push(format!("}}"));
            },

            _ => unreachable!(),
        }
    }
}

fn main() {
    let file = fs::read_to_string("test.py").expect("");
    let program = mutate(parser::parse_program(&file).unwrap(), ast::Location::new(20, 5));

    let mut graph: Vec<String> = Vec::new();

    insert_node(&"0".to_string(), "program", &mut graph);

    walk_suite(&program.statements, &"0".to_string(), &mut graph);

    println!("graph {{");
    //println!("    rankdir=LR;");

    for line in graph {
        println!("    {}", line);
    }

    println!("}}");

    //println!("\n\nDa program:\n{}", file);
}