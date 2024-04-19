fn get_fizzbuzz_v1(number: usize) -> String {
    if (number % 3 == 0) && (number % 5 == 0) {
        "FizzBuzz".to_string()
    } else if number % 3 == 0 {
        "Fizz".to_string()
    } else if number % 5 == 0 {
        "Buzz".to_string()
    } else {
        number.to_string()
    }
}

fn get_fizzbuzz_v2(number: usize) -> String {
    let mut s = String::new();
    if number % 3 == 0 {
        s.push_str("Fizz");
    }
    if number % 5 == 0 {
        s.push_str("Buzz");
    }
    if s.is_empty() {
        s = number.to_string();
    }
    s
}

fn get_fizzbuzz_v3(number: usize) -> String {
    fn add_str_if_divisible(str: &mut String, dividend: usize, divisor: usize, str_to_add: &str) {
        if dividend % divisor == 0 {
            str.push_str(str_to_add);
        }
    }

    let mut s = String::new();
    add_str_if_divisible(&mut s, number, 3, "Fizz");
    add_str_if_divisible(&mut s, number, 5, "Buzz");
    if s.is_empty() {
        s = number.to_string();
    }
    s
}

fn main() {
    for i in 1..=100 {
        println!("{}", get_fizzbuzz_v3(i));
    }
}
