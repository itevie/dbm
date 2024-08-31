use crate::errors::{MakerError, MakerErrorType};

use super::{
    nodes::Expression,
    scope::Scope,
    values::{self, Boolean, Null, RuntimeValue},
};

type E = Result<RuntimeValue, MakerError>;

pub struct Interpreter {
    scope: Scope,
}

macro_rules! evaluate {
    ($self:ident, $what:expr) => {
        Box::pin(async { $self.evaluate($what).await }).await
    };
}

impl Interpreter {
    pub fn new(scope: Scope) -> Self {
        Self { scope }
    }

    pub async fn evaluate(&mut self, expression: Expression) -> E {
        match expression {
            // ----- Special -----
            Expression::Block(block) => {
                let mut last: RuntimeValue = Null::make();

                for node in block.nodes {
                    last = evaluate!(self, node)?;
                }

                Ok(last)
            }
            // ----- Expressions -----
            Expression::VariableDeclaration(dec) => {
                let value = evaluate!(self, *dec.value)?;
                self.scope.declare(&dec.name.name, value)?;

                Ok(values::Null::make())
            }
            Expression::Member(expr) => {
                let left = evaluate!(self, *expr.left.clone())?;

                match left {
                    RuntimeValue::Object(obj) => {
                        if let Expression::Identifier(ref ident) = *expr.right {
                            // Check if the object has it
                            if !obj.items.contains_key(&ident.name) {
                                return Err(MakerError::lang(
                                    format!("Object does not have key {}", ident.name),
                                    expr.right.get_location(),
                                    MakerErrorType::RuntimeError,
                                ));
                            }

                            Ok(obj.items.get(&ident.name).unwrap().clone())
                        } else {
                            return Err(MakerError::lang(
                                "Can only index an object with an identifier!",
                                expr.right.get_location(),
                                MakerErrorType::RuntimeError,
                            ));
                        }
                    }
                    _ => {
                        return Err(MakerError::lang(
                            format!("Cannot index a {}", left.type_name()),
                            expr.left.get_location(),
                            MakerErrorType::RuntimeError,
                        ));
                    }
                }
            }
            Expression::Call(call) => {
                let callee = evaluate!(self, *call.callee)?;
                let mut args: Vec<RuntimeValue> = vec![];

                // Collect args
                for arg in call.args {
                    args.push(evaluate!(self, arg)?);
                }

                // Check the type of the caller
                match callee {
                    RuntimeValue::NativeFunction(func) => {
                        let value = (func.func)(args);

                        // Check if it is a future, if so, automatically await it
                        match value {
                            Ok(RuntimeValue::Future(future)) => {
                                let v2 = future.value.clone();
                                let mut value = v2.lock().await;

                                let v = value.as_mut().await;
                                Ok(v?)
                            }
                            v => Ok(v?),
                        }
                    }
                    c => {
                        return Err(MakerError::lang(
                            format!("Cannot call a {}", c.type_name()),
                            call.location,
                            MakerErrorType::RuntimeError,
                        ))
                    }
                }
            }
            Expression::Logical(expr) => {
                let left = evaluate!(self, *expr.left)?;
                let right = evaluate!(self, *expr.right)?;

                // Check if types are the same
                if left.type_name() != right.type_name() {
                    return Ok(Boolean::make(false));
                }

                let result = match (left, right) {
                    (RuntimeValue::Boolean(l), RuntimeValue::Boolean(r)) => l.value == r.value,
                    (RuntimeValue::StringValue(l), RuntimeValue::StringValue(r)) => {
                        l.value == r.value
                    }
                    (RuntimeValue::Number(l), RuntimeValue::Number(r)) => l.value == r.value,
                    (RuntimeValue::Null(_), _) => true,
                    _ => false,
                };

                Ok(Boolean::make(result))
            }
            Expression::IfBlock(block) => {
                let test = evaluate!(self, *block.test)?.is_truthy();
                println!("{}", test);

                if test {
                    evaluate!(self, Expression::Block(block.success))
                } else if let Some(alternate) = block.alternate {
                    evaluate!(self, *alternate)
                } else {
                    Ok(values::Null::make())
                }
            }
            // ----- Literals -----
            Expression::Identifier(ident) => Ok(self.scope.get(&ident.name)?),
            Expression::Number(value) => Ok(values::Number::make(value.value)),
            Expression::StringNode(string) => Ok(values::StringValue::make(string.value)),
        }
    }
}
