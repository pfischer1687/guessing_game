use rand::prelude::*;

fn main() {
    let number: i32 = thread_rng().gen_range(1..101);
    println!("{number}");
}
