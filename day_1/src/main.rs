use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt");

    println!("{:?}", contents);
}
