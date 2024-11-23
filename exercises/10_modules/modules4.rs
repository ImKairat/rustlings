// This exercise demonstrates the use of modules in Rust.
// The extern_mod module is defined in a separate file (extern_mod.rs) and its functions are used in the main function.

// TODO: Fix the compiler error related to importing an external module.
mod inner;
// use extern_mod::...; 

fn main() {
    let who_are_you = "I'm a Rustacean";
    greet(who_are_you);
}