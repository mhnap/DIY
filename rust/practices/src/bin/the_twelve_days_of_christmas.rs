fn to_adjective_str(number: u8) -> &'static str {
    match number {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => panic!(),
    }
}

fn to_numeral_str(number: u8) -> &'static str {
    match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        _ => panic!(),
    }
}

fn to_gift_str(number: u8) -> &'static str {
    match number {
        1 => "partridge in a pear tree",
        2 => "turtle doves",
        3 => "French hens",
        4 => "calling birds",
        5 => "golden rings",
        6 => "geese a-laying",
        7 => "swans a-swimming",
        8 => "maids a-milking",
        9 => "ladies dancing",
        10 => "lords a-leaping",
        11 => "pipers piping",
        12 => "drummers drumming",
        _ => panic!(),
    }
}

fn gen_gifts_for_day(day: u8) -> String {
    let mut gifts: String = String::new();
    for gift in (1..=day).rev() {
        if gift == 1 {
            if day == 1 {
                gifts.push_str(format!("A {}\n", to_gift_str(gift)).as_ref());
            } else {
                gifts.push_str(format!("And a {}\n", to_gift_str(gift)).as_ref());
            }
        } else {
            let mut numeral_str = to_numeral_str(gift).to_owned();
            make_ascii_titlecase(&mut numeral_str);
            gifts.push_str(format!("{} {}\n", numeral_str, to_gift_str(gift)).as_ref());
        }
    }
    gifts
}

fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

fn gen_lyrics() -> String {
    let mut lyrics = String::new();
    for day in 1..=12 {
        lyrics.push_str(
            format!(
                "On the {} day of Christmas, my true love sent to me\n{}\n",
                to_adjective_str(day),
                gen_gifts_for_day(day)
            )
            .as_ref(),
        );
    }
    lyrics
}

fn main() {
    println!(
        "\"The Twelve Days of Christmas\" carol lyrics:\n\n{}",
        gen_lyrics()
    );
}
