use std::env;
use std::process;

use euler;

fn usage(err: Option<&str>) {
    let args: Vec<String> = env::args().collect();
    match err {
        Some(err) => eprintln!("{err}"),
        None => (),
    }
    eprintln!("Usage: {} <problem number>", args[0])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(None);
        process::exit(1);
    }
    let problem = &args[1];
    let problem = problem.parse::<u32>().unwrap_or_else(|err| {
        usage(Some(&err.to_string()));
        process::exit(1);
    });
    euler::solve_problem(problem).unwrap_or_else(|err| {
        usage(Some(err));
        process::exit(1);
    });
}
