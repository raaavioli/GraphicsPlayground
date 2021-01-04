#[macro_use] extern crate failure;
#[macro_use] extern crate renderer_derive;
extern crate gl;
extern crate nalgebra;
extern crate glutin;
extern crate image;

pub mod resources;
pub mod renderer;

#[allow(arithmetic_overflow)]
fn main() {
    if let Err(e) = renderer::run() {
        println!("{}", failure_to_string(e));
    }
}

/**
* Potentially slow method, don't call in hot call site
*/
pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result: String = "Error: ".into();
    for (i, cause) in e.iter_chain().collect::<Vec<_>>().into_iter().rev().enumerate() {
        if i > 0 {
            let _ = write!(&mut result, "Caused: ");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if !backtrace_str.is_empty() {
                let _ = write!(&mut result, ", at {}", backtrace_str);
            }
        }
        let _ = writeln!(&mut result);
    }
    result
}