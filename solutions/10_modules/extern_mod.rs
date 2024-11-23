// Removed 'pub' from before 'fn'.
fn parse_txt(txt: &str) -> &str {
    let res = txt.split(" ").collect::<Vec<_>>();
    return res[res.len()-1];
}

// Added 'pub' to 'fn' to make it accessible outside the module.
pub fn greet(txt: &str) {
    let name = parse_txt(txt);
    println!("Hello, {}! 🦀🦀🦀", name);
}