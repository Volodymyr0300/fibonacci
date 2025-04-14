use std::io;

fn main() {
    println!("Enter the position (n) in the Fibonacci sequence:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    let n: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive number.");
            return;
        }
    };

    let fib = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, fib);
}

fn fibonacci(n: u32) -> u64 {
    if n ==0 {
        return 0;
    } else if n == 1 {
        return 1;
    } 

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}
