fn main() {
    let n: u8 = 12;
    println!(
        "Fibonacci {n}th number (recursive) = {}",
        my_practices::fibonacci::number_recursive(n)
    );
    println!(
        "Fibonacci {n}th number (iterative) = {}",
        my_practices::fibonacci::number_iterative(n)
    );
    println!("Fibonacci sequence to {n}th number = {:?}", my_practices::fibonacci::sequence(n));
}
