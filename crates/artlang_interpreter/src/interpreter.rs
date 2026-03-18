use std::{cell::RefCell, rc::Rc};

use artlang_ast::{
    Block, Name, Program, expression::Expression, operators::BinaryOperator, statement::Statement,
};

use crate::{environment::Environment, value::Value};

pub enum InterpreterError {
    Return(Value),
    Runtime(String),
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new_global();

        Interpreter { environment }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        match self.execute_block(program) {
            Ok(()) => Ok(()),
            Err(InterpreterError::Return(_)) => Ok(()),
            Err(InterpreterError::Runtime(message)) => Err(message),
        }
    }

    pub fn execute_block(&mut self, block: &Block) -> Result<(), InterpreterError> {
        for statement in block {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    pub fn execute_statement(&mut self, statement: &Statement) -> Result<(), InterpreterError> {
        match statement {
            Statement::Assignment(name, expression) => {
                let value = self.evaluate_expression(expression)?;
                self.environment.borrow().assign(name, value);
            }
            Statement::FunctionCall(name, args) => match name.as_str() {
                "print" => {
                    let parts: Vec<String> = args
                        .iter()
                        .map(|v| match v {
                            _ => format!("{v}"),
                        })
                        .collect();
                    let output = parts.join("\t");
                    print!("{output}");
                }
                other => println!("Function '{other}' not found"),
            },
            other => panic!("Interpreter::execute_statement ({other:?}) not implemented!"),
        }

        Ok(())
    }

    pub fn evaluate_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Value, InterpreterError> {
        match expression {
            Expression::Number(n) => Ok(Value::Integer(*n as i64)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::BinaryOperator(op, lhs, rhs) => match op {
                BinaryOperator::Multiply => {
                    let left = self.evaluate_expression(lhs)?;
                    let right = self.evaluate_expression(rhs)?;

                    // TODO: Just a test
                    Ok(left)
                }
                other => panic!(
                    "Interpreter::evaluate_expression::binary_operator: value {other:?} not implemented!"
                ),
            },
            other => panic!("Interpreter::evaluate_expression: value ({other:?}) not implemented!"),
        }
    }
}
