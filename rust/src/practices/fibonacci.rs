fn fibonacci_recursive(n: u8) -> usize {
    if n < 2 {
        n as usize
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn fibonacci_iterative(n: u8) -> usize {
    let mut prev_number = 0;
    let mut number = if n == 0 { 0 } else { 1 };
    for _ in 1..n {
        let tmp = number;
        number += prev_number;
        prev_number = tmp;
    }
    number
}

fn fibonacci_sequence(n: u8) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }
    let mut sequence = vec![0; n as usize + 1];
    sequence[1] = 1;
    for i in 2..=n as usize {
        sequence[i] = sequence[i - 1] + sequence[i - 2];
    }
    sequence
}

fn main() {
    let n: u8 = 12;
    println!(
        "Fibonacci recursive of {n}th number = {}",
        fibonacci_recursive(n)
    );
    println!(
        "Fibonacci iterative of {n}th number = {}",
        fibonacci_iterative(n)
    );
    println!(
        "Fibonacci sequence to {n}th number = {:?}",
        fibonacci_sequence(n)
    );
}
