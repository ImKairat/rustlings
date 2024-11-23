mod inner;
use extern_mod::greet; // Added the public function 'greet'for access in the main function.

fn main() {
    let who_are_you = "I'm a Rustacean";
    greet(who_are_you);
}