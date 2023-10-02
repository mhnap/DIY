use std::collections::{BTreeMap, HashMap};
use std::io;

fn read_integers() -> Vec<i32> {
    println!("Please input a list of integers separated by space: ");
    let mut integers: Vec<i32>;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        integers = input
            .split_whitespace()
            .filter_map(|c| c.parse().ok())
            .collect();
        if integers.is_empty() {
            println!("Failed to read list.");
            continue;
        }

        println!("Your list of integers is: {:?}", integers);
        break;
    }

    integers
}

fn get_mean(integers: &[i32]) -> Option<f64> {
    if integers.is_empty() {
        return None;
    }

    let sum: i32 = integers.iter().sum();
    let mean: f64 = sum as f64 / integers.len() as f64;

    Some(mean)
}

fn get_median(integers: &[i32]) -> Option<i32> {
    if integers.is_empty() {
        return None;
    }

    let mut integers = integers.to_owned();
    integers.sort();

    Some(integers[integers.len() / 2])
}

fn get_mode(integers: &[i32]) -> Option<Vec<i32>> {
    if integers.is_empty() {
        return None;
    }

    let mut key_to_occurrences: HashMap<i32, i32> = HashMap::new();
    for &i in integers {
        let occurrence = key_to_occurrences.entry(i).or_insert(0);
        *occurrence += 1;
    }

    let mut occurrence_to_keys: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for (key, occurrence) in key_to_occurrences {
        let entry = occurrence_to_keys.entry(occurrence).or_default();
        entry.push(key);
    }

    // Here `occurrence_to_keys` always have at least one element.
    Some(occurrence_to_keys.pop_last().unwrap().1)
}

fn main() {
    let integers = read_integers();

    let mean = get_mean(&integers);
    match mean {
        Some(mean) => println!("Mean is {mean}!"),
        None => println!("No mean!"),
    }

    let median = get_median(&integers);
    match median {
        Some(median) => println!("Median is {median}!"),
        None => println!("No median!"),
    }

    let mode = get_mode(&integers);
    match mode {
        Some(mode) => println!("Mode is {:?}!", mode),
        None => println!("No mode!"),
    }
}
