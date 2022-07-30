use neon::{prelude::*, result::Throw};
use rlua::{
    Error::ExternalError, FromLua, Function, Lua, MetaMethod, Result, UserData, UserDataMethods,
    Value, Variadic,
};
use std::error::Error;
use std::iter::FromIterator;
use std::sync::Arc;
use std::{fmt, str};

// im sorry for anyone who reads this code

struct RLua(Lua);

impl Finalize for RLua {
    fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
        self.0.gc_stop();
    }
}

#[derive(Debug)]
struct CustomError {
    details: String,
}

impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        &self.details
    }
}

unsafe impl Send for CustomError {}
unsafe impl Sync for CustomError {}

fn new_lua_instance(mut cx: FunctionContext) -> JsResult<JsBox<RLua>> {
    let lua = Lua::new();
    lua.context(|ctx| {
        let globals = ctx.globals();
        let print = ctx
            .create_function(|_, msg: String| {
                println!("{}", msg);
                Ok(())
            })
            .unwrap();
        globals.set("print", print)
    });
    Ok(cx.boxed(RLua(lua)))
}

fn load_lua(mut cx: FunctionContext) -> JsResult<JsObject> {
    let lua = cx.argument::<JsBox<RLua>>(0)?;
    let script = cx.argument::<JsString>(1)?;
    let result = cx.empty_object();
    let null = cx.null();
    lua.0.context(|ctx| {
        match ctx.load(&script.value(&mut cx)).exec() {
            Ok(()) => result.set(&mut cx, "err", null),
            Err(err) => {
                let msg = cx.string(format!("{}", err));
                result.set(&mut cx, "err", msg)
            }
        };
    });
    Ok(result)
}

fn add_value(mut cx: FunctionContext) -> JsResult<JsObject> {
    let lua = cx.argument::<JsBox<RLua>>(0)?;
    let key = cx.argument::<JsString>(1)?;
    let value = cx.argument::<JsString>(2)?;
    let result = cx.empty_object();
    let null = cx.null();
    lua.0.context(|ctx| {
        let globals = ctx.globals();
        match globals.set(key.value(&mut cx), value.value(&mut cx)) {
            Ok(()) => result.set(&mut cx, "err", null),
            Err(err) => {
                let msg = cx.string(format!("{}", err));
                result.set(&mut cx, "err", msg)
            }
        }
    });
    Ok(result)
}

fn call(mut cx: FunctionContext) -> JsResult<JsObject> {
    let lua = cx.argument::<JsBox<RLua>>(0)?;
    let name = cx.argument::<JsString>(1)?;
    let raw_name = name.value(&mut cx);
    let result = cx.empty_object();
    let null = cx.null();
    lua.0
        .context(|ctx| match ctx.load(&raw_name).eval::<Value>() {
            Ok(x) => match x {
                Value::Integer(x) => {
                    let node_int = cx.number(x as i32);
                    result.set(&mut cx, "value", node_int);
                    result.set(&mut cx, "err", null);
                }
                Value::Number(x) => {
                    let node_num = cx.number(x);
                    result.set(&mut cx, "value", node_num);
                    result.set(&mut cx, "err", null);
                }
                Value::String(x) => {
                    let bytes = x.as_bytes();
                    let string = str::from_utf8(&bytes).unwrap().to_string();
                    let node_str = cx.string(string);
                    result.set(&mut cx, "value", node_str);
                    result.set(&mut cx, "err", null);
                }
                Value::Boolean(x) => {
                    let node_bool = cx.boolean(x);
                    result.set(&mut cx, "value", node_bool);
                    result.set(&mut cx, "err", null);
                }
                _ => panic!("PANIC AT THE DISCO!"),
            },
            Err(err) => {
                let msg = cx.string(format!("{}", err));
                result.set(&mut cx, "err", msg);
                result.set(&mut cx, "value", null);
            }
        });
    Ok(result)
}

//fn add_function(mut cx: FunctionContext) -> JsResult<JsObject> {
//    let lua = cx.argument::<JsBox<RLua>>(0)?;
//    let name = cx.argument::<JsString>(1)?;
//    let fun = cx.argument::<JsFunction>(2)?;
//    let num_of_args = cx.argument::<JsNumber>(3)?;
//    let num_args = Box::new(num_of_args.value(&mut cx)).clone();
//    let argss: &'static mut f64 = Box::leak(num_args);
//    let result = cx.empty_object();
//    let null = cx.null();
//    lua.0.context(|ctx| {
//        //let num_args = num_of_args_rs.clone();
//        let lua_fun = ctx.create_function(|_, args: Variadic<Value>| {
//            let len = args.len() as f64;
//            if *num_args > len {
//                Err(ExternalError(Arc::new(CustomError::new(
//                    "not enough arguments",
//                ))))
//            } else if *num_args < len {
//                Err(ExternalError(Arc::new(CustomError::new(
//                    "too many arguments",
//                ))))
//            } else {
//                //let called = fun.construct_with(&mut cx);
//                Ok(())
//            }
//        });
//    });
//    Ok(result)
//}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("new", new_lua_instance)?;
    cx.export_function("load", load_lua)?;
    cx.export_function("value", add_value)?;
    cx.export_function("call", call)?;
    Ok(())
}
