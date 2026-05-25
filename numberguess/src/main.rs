use rand::RngExt;
use std::io;
fn main() {
    let mut rng = rand::rng();
    let a: i8 = rng.random_range(0..2);
    let mut input_text = String::new();
    println!("input either 0 or 1: ");
    io::stdin().read_line(&mut input_text).unwrap();//.expect("Failed to read line");
    let num: i8 = input_text.trim().parse().unwrap();
    //println!("randomnum = {}", a);

    if num == a{
        println!("i picked {} too, you guessed right", a);
    }
    else{
        println!("i picked {}, you guessed wrong", a);
    }
}
