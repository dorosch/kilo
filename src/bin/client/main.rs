use std::io;


fn main() {
    let mut username = String::new();

    io::stdin().read_line(&mut username).unwrap();

    let me = kilo::User::new(username, None);

    println!("{}", me);
}
