use std::io;

fn main() {
    let mut train = String::new();

    io::stdin()
        .read_line(&mut train)
        .expect("Failed to read line");

    // https://programming-idioms.org/idiom/41/reverse-a-string/402/rust
    let train = train
                     .chars()
                     .rev()
                     .collect::<String>();

   println!("{}", train); 
}
