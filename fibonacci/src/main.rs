use std::io;

fn main() {
    println!("Please input N.");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("N must be an integer >= 0.");

    println!("{}", fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
