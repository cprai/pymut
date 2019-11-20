use rustpython_parser::ast;

pub fn stringify_expression(expr: &ast::ExpressionType) -> &str {
    match expr {
        ast::ExpressionType::BoolOp        {op: _, values: _}                  => "BoolOp       ",
        ast::ExpressionType::Binop         {a: _, op: _, b: _}                 => "Binop        ",
        ast::ExpressionType::Subscript     {a: _, b: _}                        => "Subscript    ",
        ast::ExpressionType::Unop          {op: _, a: _}                       => "Unop         ",
        ast::ExpressionType::Await         {value: _}                          => "Await        ",
        ast::ExpressionType::Yield         {value: _}                          => "Yield        ",
        ast::ExpressionType::YieldFrom     {value: _}                          => "YieldFrom    ",
        ast::ExpressionType::Compare       {vals: _, ops: _}                   => "Compare      ",
        ast::ExpressionType::Attribute     {value: _, name: _}                 => "Attribute    ",
        ast::ExpressionType::Call          {function: _, args: _, keywords: _} => "Call         ",
        ast::ExpressionType::Number        {value: _}                          => "Number       ",
        ast::ExpressionType::List          {elements: _}                       => "List         ",
        ast::ExpressionType::Tuple         {elements: _}                       => "Tuple        ",
        ast::ExpressionType::Dict          {elements: _}                       => "Dict         ",
        ast::ExpressionType::Set           {elements: _}                       => "Set          ",
        ast::ExpressionType::Comprehension {kind: _, generators: _}            => "Comprehension",
        ast::ExpressionType::Starred       {value: _}                          => "Starred      ",
        ast::ExpressionType::Slice         {elements: _}                       => "Slice        ",
        ast::ExpressionType::String        {value: _}                          => "String       ",
        ast::ExpressionType::Bytes         {value: _}                          => "Bytes        ",
        ast::ExpressionType::Identifier    {name: _}                           => "Identifier   ",
        ast::ExpressionType::Lambda        {args: _, body: _}                  => "Lambda       ",
        ast::ExpressionType::IfExpression  {test: _, body: _, orelse: _}       => "IfExpression ",
        ast::ExpressionType::True          {}                                  => "True         ",
        ast::ExpressionType::False         {}                                  => "False        ",
        ast::ExpressionType::None          {}                                  => "None         ",
        ast::ExpressionType::Ellipsis      {}                                  => "Ellipsis     ",
    }
}

pub fn stringify_statement(stmt: &ast::StatementType) -> &str {
    match stmt {
        ast::StatementType::Break              {}                                                                      => "Break      ",
        ast::StatementType::Continue           {}                                                                      => "Continue   ",
        ast::StatementType::Return             {value: _}                                                              => "Return     ",
        ast::StatementType::Import             {names: _}                                                              => "Import     ",
        ast::StatementType::ImportFrom         {level: _, module: _, names: _}                                         => "ImportFrom ",
        ast::StatementType::Pass               {}                                                                      => "Pass       ",
        ast::StatementType::Assert             {test: _, msg: _}                                                       => "Assert     ",
        ast::StatementType::Delete             {targets: _}                                                            => "Delete     ",
        ast::StatementType::Assign             {targets: _, value: _}                                                  => "Assign     ",
        ast::StatementType::AugAssign          {target: _, op: _, value: _}                                            => "AugAssign  ",
        ast::StatementType::AnnAssign          {target: _, annotation: _, value: _}                                    => "AnnAssign  ",
        ast::StatementType::Expression         {expression: _}                                                         => "Expression ",
        ast::StatementType::Global             {names: _}                                                              => "Global     ",
        ast::StatementType::Nonlocal           {names: _}                                                              => "Nonlocal   ",
        ast::StatementType::If                 {test: _, body: _, orelse: _}                                           => "If         ",
        ast::StatementType::While              {test: _, body: _, orelse: _}                                           => "While      ",
        ast::StatementType::With               {is_async: _, items: _, body: _}                                        => "With       ",
        ast::StatementType::For                {is_async: _, target: _, iter: _, body: _, orelse: _}                   => "For        ",
        ast::StatementType::Raise              {exception: _, cause: _}                                                => "Raise      ",
        ast::StatementType::Try                {body: _, handlers: _, orelse: _, finalbody: _}                         => "Try        ",
        ast::StatementType::ClassDef           {name: _, body: _, bases: _, keywords: _, decorator_list: _}            => "ClassDef   ",
        ast::StatementType::FunctionDef        {is_async: _, name: _, args: _, body: _, decorator_list: _, returns: _} => "FunctionDef",
    }
}