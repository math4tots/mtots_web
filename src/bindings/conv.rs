use mtots::Value;
use mtots::rterr;
use mtots::Globals;
use mtots::Result;

fn from_js(globals: &mut Globals, value: stdweb::Value) -> Result<Value> {
    match value {
        stdweb::Value::Null | stdweb::Value::Undefined => Ok(Value::Nil),
        stdweb::Value::Bool(x) => Ok(Value::Bool(x)),
        stdweb::Value::Number(x) => Ok(Value::Number(x.into())),
        stdweb::Value::Symbol(x) => Err(rterr!("TODO: Symbol value ({:?})", x)),
        stdweb::Value::String(x) => Ok(Value::String(x.into())),
        stdweb::Value::Reference(r) => Ok(globals.new_handle::<stdweb::Reference>(r)?.into()),
    }
}
