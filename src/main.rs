fn problem1() {
    // If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let m3: u32 = (3..1000).step_by(3).sum();
    let m5: u32 = (5..1000).step_by(5).sum();
    let m15: u32 = (15..1000).step_by(15).sum();
    let result = m3 + m5 - m15;
    println!("{result}")
}

fn main() {
    problem1()
}
