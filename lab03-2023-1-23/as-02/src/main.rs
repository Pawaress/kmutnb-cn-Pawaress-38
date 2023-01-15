fn main() {
    println!("{}",algorithm(100));
}
fn algorithm(x: i32) -> i32 {
    let mut susm = 0;
    for i in 0..x + 1 {
        susm = susm + i
    }
    susm
}