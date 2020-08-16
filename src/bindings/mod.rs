mod conv;
use mtots::rterr;
use mtots::Result;
use mtots::NativeModule;
use stdweb::js;
use stdweb::web;

pub const NAME: &str = "a.web";

pub(crate) fn new() -> NativeModule {
    NativeModule::new(NAME, |m| {
        m.class::<stdweb::Reference, _>("JsRef", |cls| {
            cls.getattr(|globals, handle, attrname| {
                let handle = handle.borrow().clone();
                let result = js! {
                    return @{handle}[@{attrname}];
                };
                Ok(Some(conv::from_js(globals, result)?))
            });
            cls.setattr(|globals, handle, attrname, value| {
                let handle = handle.borrow().clone();
                let value = conv::to_js(globals, value)?;
                js! {@(no_return)
                    @{handle}[@{attrname}] = @{value};
                };
                Ok(())
            });
            cls.method_call(|globals, handle, method_name, args, kwargs| {
                if kwargs.is_some() {
                    return Err(rterr!("kwargs are not allowed on JsRef methods"));
                }
                let handle = handle.borrow().clone();
                let args = args.into_iter().map(|x| conv::to_js(globals, x)).collect::<Result<Vec<_>>>()?;
                let result = js! {
                    const owner = @{handle};
                    const method = owner[@{method_name}];
                    return method.apply(owner, @{args});
                };
                conv::from_js(globals, result)
            });
        });
        m.func("document", [], "", |globals, _, _| {
            Ok(globals.new_handle::<stdweb::Reference>(web::document().into())?.into())
        });
        m.func("window", [], "", |globals, _, _| {
            Ok(globals.new_handle::<stdweb::Reference>(web::window().into())?.into())
        });
    })
}
