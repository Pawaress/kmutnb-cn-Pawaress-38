use std::io::{self,Write};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Failed to parse input");
    let mut n = input;
    print(n);
}
fn print(x: i32){
    let mut nub=0;
    for con in 0..x{
        for col in 0..x{
            if con==col || col==0 || col==x-1 {
                print!("X ")
            }else {
                print!("O ")
            }
        }
        println!(" ")
    }
}