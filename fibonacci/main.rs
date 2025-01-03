use std::io;

fn main() {
    println!("Generate the nth Fibonacci number (F to the n)");
    println!("Please input n: ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number greater than or equal to 0");
            return
        }
    };

    if n == 0 {
        println!("\n0");
    } else {
        let f: u32 = do_fib(n);
        println!("\n{f}");
    }
}

fn do_fib(n: u32) -> u32 {
    if n <= 1 { return 1 }
    do_fib(n - 1) + do_fib(n - 2)
}
