fn problem1() {
    // If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let m3: u32 = (3..1000).step_by(3).sum();
    let m5: u32 = (5..1000).step_by(5).sum();
    let m15: u32 = (15..1000).step_by(15).sum();
    let result = m3 + m5 - m15;
    println!("Problem 1: {result}")
}

fn problem2() {
    // Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
    // 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
    // By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
    let mut tc = 1;
    let mut tp = 0;
    let mut sum = 0;
    loop {
        if tc % 2 == 0 {
            sum += tc
        }
        let _tc = tc;
        tc = tc + tp;
        tp = _tc;

        if tc >= 4000000 {
            break
        }
    }
    let result = sum;
    println!("Problem 2: {result}")
}

fn main() {
    problem1();
    problem2();
}
