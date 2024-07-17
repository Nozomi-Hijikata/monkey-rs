use crate::ast::{Expr, Node, Program, Stmt};
use crate::object::{Integer, Object};

fn eval(node: &dyn Node) -> Box<dyn Object> {
    node.eval()
}

impl Node for Program {
    fn eval(&self) -> Box<dyn Object> {
        let mut result: Box<dyn Object> = Box::new(Integer { value: 0 }); //TODO: NULL objectにする
        for stmt in &self.statements {
            result = eval(stmt);
        }
        result
    }
}

impl Node for Box<Stmt> {
    fn eval(&self) -> Box<dyn Object> {
        match self {
            Stmt::Let { name, value } => {
                println!("Let statement: {} = {:?}", name, value);
                // value.eval()
            }
            Stmt::Return { return_value } => {
                println!("Return statement: {:?}", return_value);
                // return_value.eval()
            }
            Stmt::Expr { expression } => {
                println!("Expression statement: {:?}", expression);
                // expression.eval()
            }
            Stmt::Block { statements } => {
                println!("Block statement: {:?}", statements);
                // for stmt in statements {
                //     eval(stmt);
                // }
                // Box::new(0)
            }
        }
    }
}
