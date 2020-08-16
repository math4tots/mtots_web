use mtots::ordie;
use mtots::Globals;
use mtots::RcStr;
use mtots::Result;
use mtots::Source;
use std::rc::Rc;
// use stdweb::traits::*;
// use stdweb::unstable::TryInto;
// use stdweb::web::event::{MouseMoveEvent, ResizeEvent};
// use stdweb::web::html_element::CanvasElement;
// use stdweb::web::{document, window, CanvasRenderingContext2d};

// Shamelessly stolen from webplatform's TodoMVC example.
// macro_rules! enclose {
//     ( ($( $x:ident ),*) $y:expr ) => {
//         {
//             $(let $x = $x.clone();)*
//             $y
//         }
//     };
// }

macro_rules! add_src {
    ($globals:ident, $name:expr) => {
        $globals.add_custom_source(Rc::new(Source::new(
            $name.into(),
            None,
            include_str!(concat!("msrcs/", $name, ".u")).into(),
        )))?;
    };
}

fn add_srcs(globals: &mut Globals) -> Result<()> {
    add_src!(globals, "__main");
    add_src!(globals, "demos.draw");
    add_src!(globals, "demos.pixel");
    add_src!(globals, "demos.code");
    add_src!(globals, "demos.code.sample");
    Ok(())
}

pub fn main() {
    stdweb::initialize();

    // Callbacks require access to Globals, but there's no real good
    // way to get it for them.
    // So we just leak it.
    let globals = Box::leak(Box::new(mtots::Globals::new()));

    let r = globals.add_native_module(crate::bindings::new());
    ordie(globals, r);

    globals.set_print(|string| {
        stdweb::console!(log, string);
    });

    globals.set_eprint(|string| {
        stdweb::console!(error, string);
    });

    let r = add_srcs(globals);
    ordie(globals, r);
    mtots::add_standard_modules(globals);

    // let canvas: CanvasElement = document()
    //     .query_selector("#canvas")
    //     .unwrap()
    //     .unwrap()
    //     .try_into()
    //     .unwrap();
    // let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    // canvas.set_width(canvas.offset_width() as u32);
    // canvas.set_height(canvas.offset_height() as u32);

    // window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
    //     canvas.set_width(canvas.offset_width() as u32);
    //     canvas.set_height(canvas.offset_height() as u32);
    // }));

    // canvas.add_event_listener(enclose!( (context) move |event: MouseMoveEvent| {
    //     context.fill_rect(f64::from(event.client_x() - 5), f64::from(event.client_y() - 5)
    //                       , 10.0, 10.0);
    // }));

    let module = RcStr::from("__main");
    globals.set_main(module.clone());
    let r = globals.load(&module).map(|_| ());
    ordie(globals, r);

    stdweb::event_loop();
}
