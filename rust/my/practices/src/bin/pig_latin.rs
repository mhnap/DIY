use std::io;
use std::io::Write;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const VOWEL_SUFFIX: &str = "hay";
const CONSONANT_SUFFIX: &str = "ay";

fn is_vowel(ch: char) -> bool {
    let ch = ch.to_ascii_lowercase();
    VOWELS.contains(&ch)
}

fn str_to_pig_latin(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut consonant_cluster = String::new();
    let mut s_chars = s.chars();
    let mut s_prev_chars = s.chars();

    // Collect consonant cluster if any.
    loop {
        match s_chars.next() {
            Some(ch) if !is_vowel(ch) => {
                consonant_cluster.push(ch);
                s_prev_chars.next();
            }
            _ => break,
        };
    }

    // Handle case when last char is uppercase.
    fn push_suffix(str: &mut String, suffix: &str) {
        // Note, `unwrap` call is safe here, because we checked above for emptiness.
        if str.chars().last().unwrap().is_uppercase() {
            str.push_str(&suffix.to_uppercase());
        } else {
            str.push_str(suffix);
        }
    }

    if !consonant_cluster.is_empty() {
        let mut new_s: String = s_prev_chars.collect();
        new_s.push_str(&consonant_cluster);
        // Note, `unwrap` call is safe here, because we checked above for emptiness.
        push_suffix(&mut new_s, CONSONANT_SUFFIX);
        new_s
    } else {
        let mut new_s = s.to_owned();
        push_suffix(&mut new_s, VOWEL_SUFFIX);
        new_s
    }
}

fn main() {
    loop {
        print!("Please input your string (`q` to quit): ");
        io::stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "q" {
            break;
        }

        println!("Your pig latin string: {}", str_to_pig_latin(input));
    }
}
