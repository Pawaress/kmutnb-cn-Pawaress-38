use std::io::{self,Write};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Failed to parse input");
    let mut n = input;
    lperamitup(n);
}
fn lperamitup(x: i32){
    let can= x-1;
    let mut addcen=x-1;
    let mut delete=x-1;
    for row in 0..x{
        for col in 0..(x*2)-1{
            if (col<=addcen) && (col>=delete){
                print!("* ");
                
            }else{
                print!("  ");
            }
            if col == (x*2)-1-1{
                delete =delete-1;
                addcen=addcen+1;
            }
        }
        println!("");
    }
}