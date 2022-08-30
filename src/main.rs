fn problem1() {
    // If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let mut m3 = Vec::new();
    let mut m5 = Vec::new();
    let mut m15 = Vec::new();
    for i in (3..1000).step_by(3) {
        m3.push(i)
    }
    for i in (5..1000).step_by(5) {
        m5.push(i)
    }
    for i in (15..1000).step_by(15) {
        m15.push(i)
    }
    let result: i32 = m3.iter().sum::<i32>() + m5.iter().sum::<i32>() - m15.iter().sum::<i32>();
    println!("{result}")
}

fn main() {
    problem1()
}
