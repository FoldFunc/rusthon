use std::io;

pub fn input(s: String) -> String {
    println!("{}", s);
    let mut answ = String::new();

    io::stdin()
        .read_line(&mut answ)
        .expect("Error reading from input");

    let answ = answ.trim();
    return answ.to_string();
}
