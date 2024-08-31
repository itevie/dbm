use std::{collections::HashMap, fmt, future::Future, pin::Pin, sync::Arc};

use tokio::sync::Mutex;

use crate::errors::MakerError;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Null(Null),
    StringValue(StringValue),
    Number(Number),
    NativeFunction(NativeFunction),
    Future(MakerFuture),
    Object(Object),
    Boolean(Boolean),
}

unsafe impl Send for RuntimeValue {}
unsafe impl Sync for RuntimeValue {}

impl RuntimeValue {
    pub fn type_name(&self) -> &str {
        match self {
            RuntimeValue::Future(_) => "future",
            RuntimeValue::NativeFunction(_) => "native_function",
            RuntimeValue::Null(_) => "null",
            RuntimeValue::Number(_) => "number",
            RuntimeValue::Object(_) => "object",
            RuntimeValue::StringValue(_) => "string",
            RuntimeValue::Boolean(_) => "boolean",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            RuntimeValue::Number(v) => v.value > 0f64,
            RuntimeValue::StringValue(v) => v.value.len() != 0,
            RuntimeValue::Boolean(v) => v.value,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RuntimeValue::Future(_) => "future".to_string(),
            RuntimeValue::NativeFunction(_) => "function".to_string(),
            RuntimeValue::Null(_) => "null".to_string(),
            RuntimeValue::Number(v) => v.value.to_string(),
            RuntimeValue::Object(_) => "object".to_string(),
            RuntimeValue::StringValue(v) => v.value.to_string(),
            RuntimeValue::Boolean(v) => v.value.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Null {}

impl Null {
    pub fn make() -> RuntimeValue {
        RuntimeValue::Null(Null {})
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn make(value: f64) -> RuntimeValue {
        RuntimeValue::Number(Number { value })
    }
}

#[derive(Debug, Clone)]
pub struct StringValue {
    pub value: String,
}

impl StringValue {
    pub fn make(value: String) -> RuntimeValue {
        RuntimeValue::StringValue(StringValue { value })
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn make(value: bool) -> RuntimeValue {
        RuntimeValue::Boolean(Boolean { value })
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub items: HashMap<String, RuntimeValue>,
}

impl Object {
    pub fn make(items: HashMap<String, RuntimeValue>) -> RuntimeValue {
        RuntimeValue::Object(Object { items })
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub func: Arc<dyn Fn(Vec<RuntimeValue>) -> Result<RuntimeValue, MakerError>>,
}

unsafe impl Send for NativeFunction {}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "native_function")
    }
}

#[derive(Clone)]
pub struct MakerFuture {
    pub value: Arc<Mutex<Pin<Box<dyn Future<Output = Result<RuntimeValue, MakerError>> + Send>>>>,
}

impl fmt::Debug for MakerFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "native_function")
    }
}
