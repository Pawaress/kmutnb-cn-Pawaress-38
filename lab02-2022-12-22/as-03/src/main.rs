use std::io::{self,Write};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Failed to parse input");
    let mut n = input;
    lperamitdown(n);
}
fn lperamitdown(x: i32){
    let can= x-1;
    let mut addcen=(x*2)-1-1;
    let mut delete=1-1;
    for row in 0..x{
        for col in 0..(x*2)-1{
            if (col<=addcen) && (col>=delete){
                print!("* ");
                
            }else{
                print!("  ");
            }
            if col == (x*2)-1-1{
                delete =delete+1;
                addcen=addcen-1;
            }
        }
        println!("");
        
    }
}