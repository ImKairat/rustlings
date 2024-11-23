// No need to add new lines of code — just add or remove the 'pub' keyword.

pub fn parse_txt(txt: &str) -> &str {
    let res = txt.split(" ").collect::<Vec<_>>();
    return res[res.len()-1];
}


fn greet(txt: &str) {
    let name = parse_txt(txt);
    println!("Hello, {}! 🦀🦀🦀", name);
}