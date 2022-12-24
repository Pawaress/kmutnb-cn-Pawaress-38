use std::io::{self,Write};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Failed to parse input");
    let mut n = input;
    l01(n);
}
fn l01(x: i32){
    for row in 0..x{
        for col in 0..row+1{
            print!("* ");
        }
        println!("");
    }

}