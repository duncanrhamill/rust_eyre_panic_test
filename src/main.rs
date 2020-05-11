// ----
// ORIGINAL CODE
// Causes panic in backtrace when run with RUST_BACKTRACE=1/full
//
// Use "eyre" and "color-eyre" in Cargo.toml

//use eyre::eyre;
//use color_eyre::Result;

//fn main() -> Result<()> {
//    return Err(eyre!("Panicy panic"));
//}

// ----
// EYRE ONLY CODE
// Doesn't cause panic
//
// Use only "eyre" in Cargo.toml

//use eyre::{eyre, Result};
//
//fn main() -> Result<()> {
//    return Err(eyre!("Error"));
//}

// ----
// BACKTRACE DIRECT TRACE
// Doesn't cause panic on backtrace
//
// Only use "backtrace" in Cargo.toml

//extern crate backtrace;

//fn main() {
//    backtrace::trace(|frame| {
//        let ip = frame.ip();
//        let symbol_address = frame.symbol_address();
//
//        // Resolve this instruction pointer to a symbol name
//        backtrace::resolve_frame(frame, |symbol| {
//            if let Some(name) = symbol.name() {
//                println!("Name: {}", name);
//            }
//            if let Some(filename) = symbol.filename() {
//                println!("Filename: {:?}", filename);
//            }
//        });
//
//        true // keep going to the next frame
//    });
//
//    panic!("panic");
//}

// ----
// DIFFERENT BACKTRACE
// Doesn't cause panic
//
// Only use "backtrace" in Cargo.toml

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bt = backtrace::Backtrace::new();

    println!("normal stuff");

    println!("bt: {:?}", bt);

    panic!("panic!");
}
