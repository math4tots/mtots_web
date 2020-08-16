use mtots::rterr;
use mtots::Globals;
use mtots::List;
use mtots::Map;
use mtots::Result;
use mtots::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use stdweb::js;

pub(super) fn from_js(globals: &mut Globals, value: stdweb::Value) -> Result<Value> {
    match value {
        stdweb::Value::Null | stdweb::Value::Undefined => Ok(Value::Nil),
        stdweb::Value::Bool(x) => Ok(Value::Bool(x)),
        stdweb::Value::Number(x) => Ok(Value::Number(x.into())),
        stdweb::Value::Symbol(x) => Err(rterr!("TODO: Symbol value ({:?})", x)),
        stdweb::Value::String(x) => Ok(Value::String(x.into())),
        stdweb::Value::Reference(r) => Ok(globals.new_handle::<stdweb::Reference>(r)?.into()),
    }
}

pub(super) fn to_js(globals: &mut Globals, value: Value) -> Result<stdweb::Value> {
    match value {
        Value::Nil => Ok(stdweb::Value::Null),
        Value::Bool(x) => Ok(stdweb::Value::Bool(x)),
        Value::Number(x) => Ok(stdweb::Value::Number(x.into())),
        Value::String(x) => Ok(stdweb::Value::String(x.unwrap_or_clone())),
        Value::List(list) => {
            let vec = List::unwrap_or_clone(list)
                .into_iter()
                .map(|x| to_js(globals, x))
                .collect::<Result<Vec<_>>>()?;
            Ok(vec.into())
        }
        Value::Map(map) => {
            let map = Map::unwrap_or_clone(map)
                .into_iter()
                .map(|(key, val)| Ok((String::try_from(Value::from(key))?, to_js(globals, val)?)))
                .collect::<Result<HashMap<String, stdweb::Value>>>()?;
            Ok(map.into())
        }
        Value::Function(func) => {
            let globals: &mut Globals = unsafe { std::mem::transmute(globals) };
            let func = move |args: Vec<stdweb::Value>| {
                let r = args.into_iter().map(|v| from_js(globals, v)).collect::<Result<Vec<_>>>();
                let args = mtots::ordie(globals, r);
                let r = func.apply(globals, args, None);
                let ret = mtots::ordie(globals, r);
                let r = to_js(globals, ret);
                mtots::ordie(globals, r)
            };
            Ok(js! {
                return function(...args) {
                    return @{func}(args);
                }
            })
        }
        _ if value.is_handle::<stdweb::Reference>() => Ok(
            stdweb::Value::Reference(value.into_handle::<stdweb::Reference>()?.borrow().clone()),
        ),
        value => Err(rterr!("Could not convert {:?} value into a javascript value", value)),
    }
}
