// use std::time::Duration;

fn main() {
    let mut counter = 5;
    while counter > 0 {
        println!("{}", counter);
        counter -= 1;
    }
    print!("done!");
}
