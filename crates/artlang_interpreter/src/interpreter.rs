use std::{cell::RefCell, rc::Rc};

use artlang_ast::{
    Block, Program,
    expression::Expression,
    operators::{BinaryOperator, UnaryOperator},
    statement::Statement,
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

    pub fn execute_scoped_block(&mut self, block: &Block) -> Result<(), InterpreterError> {
        let parent = self.environment.clone();
        self.environment = Environment::new_child(&parent);

        let result = self.execute_block(block);

        self.environment = parent;

        result
    }

    pub fn execute_statement(&mut self, statement: &Statement) -> Result<(), InterpreterError> {
        match statement {
            Statement::Assignment(name, expression) => {
                let value = self.evaluate_expression(expression)?;
                self.environment.borrow().assign(name, value);
            }
            Statement::FunctionCall(name, args) => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.evaluate_expression(arg)?);
                }

                match name.as_str() {
                    "print" => {
                        let parts: Vec<String> = evaluated_args
                            .iter()
                            .map(|v| match v {
                                _ => format!("{v}"),
                            })
                            .collect();
                        let output = parts.join("\t");
                        print!("{output}");
                    }
                    _ => {
                        self.call_function(name, &evaluated_args)?;
                    }
                }
            }
            Statement::For {
                variable,
                start,
                limit,
                step,
                body,
            } => {
                let start_value = self.evaluate_expression(start)?;
                let limit_value = self.evaluate_expression(limit)?;
                let step_value = match step {
                    Some(s) => self.evaluate_expression(s)?,
                    None => Value::Integer(1),
                };

                let mut i = start_value
                    .as_integer()
                    .map_err(InterpreterError::Runtime)?;
                let limit = limit_value
                    .as_integer()
                    .map_err(InterpreterError::Runtime)?;
                let step = step_value.as_integer().map_err(InterpreterError::Runtime)?;

                let parent = self.environment.clone();
                self.environment = Environment::new_child(&parent);

                loop {
                    if i > limit {
                        break;
                    }

                    self.environment.borrow().set(variable, Value::Integer(i));
                    self.execute_block(body)?;

                    i += step;
                }

                self.environment = parent;
            }
            Statement::While { condition, body } => loop {
                let condition_val = self.evaluate_expression(condition)?;
                if !condition_val.is_truthy() {
                    break;
                }

                self.execute_scoped_block(body)?;
            },
            Statement::FunctionDefinition { name, params, body } => {
                let function = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    environment: self.environment.clone(),
                };
                self.environment.borrow().assign(name, function);
            }
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
            Expression::Float(n) => Ok(Value::Float(*n)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),
            Expression::Null => Ok(Value::Null),
            Expression::Variable(name) => self
                .environment
                .borrow()
                .get(name)
                .ok_or_else(|| InterpreterError::Runtime(format!("Undefined variable: {name}"))),
            Expression::BinaryOperator(op, lhs, rhs) => {
                let left = self.evaluate_expression(lhs)?;
                let right = self.evaluate_expression(rhs)?;

                let result = match op {
                    BinaryOperator::Add => Value::math_add(&left, &right),
                    BinaryOperator::Subtract => Value::math_sub(&left, &right),
                    BinaryOperator::Multiply => Value::math_mul(&left, &right),
                    BinaryOperator::Divide => Value::math_div(&left, &right),
                    BinaryOperator::IDivide => Value::math_idiv(&left, &right),
                    BinaryOperator::Modulus => Value::math_mod(&left, &right),
                    BinaryOperator::Power => Value::math_pow(&left, &right),
                    BinaryOperator::Concatenate => Value::string_concat(&left, &right),
                    BinaryOperator::LessThan => {
                        let a = left.as_integer();
                        let b = right.as_integer();

                        Ok(Value::Boolean(a < b))
                    }
                    other => panic!(
                        "Interpreter::evaluate_expression::binary_operator: value {other:?} not implemented!"
                    ),
                };

                result.map_err(InterpreterError::Runtime)
            }
            Expression::UnaryOperator(op, val) => {
                let value = self.evaluate_expression(val)?;

                let result = match op {
                    UnaryOperator::Minus => Value::math_mul(&value, &Value::Integer(-1)),
                    other => panic!(
                        "Interpreter::evaluate_expression::unary_operator: value {other:?} not implemented"
                    ),
                };

                result.map_err(InterpreterError::Runtime)
            }
            Expression::FunctionCall(name, args) => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.evaluate_expression(arg)?);
                }

                self.call_function(name, &evaluated_args)
            }
            other => panic!("Interpreter::evaluate_expression: value ({other:?}) not implemented!"),
        }
    }

    pub fn call_function(&mut self, name: &str, args: &[Value]) -> Result<Value, InterpreterError> {
        let lookup = self.environment.borrow().get(name);

        match lookup {
            Some(Value::Function {
                params,
                body,
                environment,
            }) => self.call_user_function(&params, &body, &environment, args),

            Some(other) => Err(InterpreterError::Runtime(format!(
                "Attempted to call a {} value '{name}'",
                other.type_name()
            ))),

            None => Err(InterpreterError::Runtime(format!(
                "Undefined function '{name}'"
            ))),
        }
    }

    pub fn call_user_function(
        &mut self,
        params: &[String],
        body: &Block,
        environment: &Rc<RefCell<Environment>>,
        args: &[Value],
    ) -> Result<Value, InterpreterError> {
        let caller_environment = self.environment.clone();
        let function_environment = Environment::new_child(environment);

        for (i, param) in params.iter().enumerate() {
            let value = args.get(i).cloned().unwrap_or(Value::Null);
            function_environment.borrow().set(param, value);
        }

        self.environment = function_environment;
        let result = self.execute_block(body);
        self.environment = caller_environment;

        match result {
            Ok(()) => Ok(Value::Null),
            Err(..) => Err(InterpreterError::Runtime(format!(
                "Error executing function",
            ))),
        }
    }
}
