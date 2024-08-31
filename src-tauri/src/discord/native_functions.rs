use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::Mutex;

use crate::errors::MakerError;
use crate::errors::MakerErrorType;
use crate::lang;
use crate::lang::lexer::Location;
use crate::lang::values;
use crate::lang::values::RuntimeValue;
use crate::lang::values::StringValue; // Ensure this is imported

macro_rules! create_maker_future {
    ($ctx:expr, $e:expr) => {{
        let ctx = Arc::clone(&$ctx);

        let func = move |args: Vec<RuntimeValue>| {
            let ctx2 = Arc::clone(&ctx);
            let future = async move { ($e)(ctx2, args).await };

            let value: Pin<
                Box<dyn Future<Output = Result<lang::values::RuntimeValue, MakerError>> + Send>,
            > = Box::pin(future);

            Ok(lang::values::RuntimeValue::Future(
                lang::values::MakerFuture {
                    value: Arc::new(Mutex::new(value)),
                },
            ))
        };

        lang::values::RuntimeValue::NativeFunction(lang::values::NativeFunction {
            func: Arc::from(func),
        })
    }};
}

macro_rules! get_arg {
    ($args:ident, $index:expr, $type:ident) => {{
        if $index >= $args.len() {
            return Err(MakerError::lang(
                format!(
                    "Not enough args provided! Missing arg at argument {}",
                    $index
                ),
                Location::no_location(),
                MakerErrorType::RuntimeError,
            ));
        }
        match &$args[$index] {
            RuntimeValue::$type(a) => a.clone(),
            _ => {
                return Err(MakerError::lang(
                    format!("Invalid type for argument {}", $index),
                    Location::no_location(),
                    MakerErrorType::RuntimeError,
                ))
            }
        }
    }};

    ($args:ident, $index:expr) => {{
        if $index >= $args.len() {
            return Err(MakerError::lang(
                format!(
                    "Not enough args provided! Missing arg at argument {}",
                    $index
                ),
                Location::no_location(),
                MakerErrorType::RuntimeError,
            ));
        }
        $args[$index].clone()
    }};
}

struct MakerContext {
    pub ctx: Arc<Context>,
    pub msg: Arc<Message>,
}

pub fn generate_from_message(ctx: Arc<Context>, msg: Arc<Message>) -> RuntimeValue {
    let context = Arc::from(MakerContext { ctx, msg });
    values::Object::make(HashMap::from([
        (
            "reply".to_string(),
            create_maker_future!(
                Arc::clone(&context),
                |ctx2: Arc<MakerContext>, args: Vec<RuntimeValue>| async move {
                    let result = ctx2
                        .msg
                        .reply(&ctx2.ctx.http, get_arg!(args, 0).to_string())
                        .await
                        .unwrap();
                    let _msg2 = generate_from_message(ctx2.ctx.clone(), Arc::from(result));
                    Ok(lang::values::Null::make())
                }
            ),
        ),
        (
            "id".to_string(),
            StringValue::make(context.msg.id.to_string()),
        ),
        (
            "content".to_string(),
            StringValue::make(context.msg.content.to_string()),
        ),
    ]))
}
