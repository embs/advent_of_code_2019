use std::collections::HashMap;

fn main() {
    let mut satisfying_passwords_count = 0;

    'outer: for p in 284639..=748759 {
    //'outer: for p in 666666..=748759 {
        let mut password = p.to_string();

        // It is a six-digit number.
        if password.len() != 6 { continue; }

        let mut previous_char = password.remove(0);
        let mut chars_count = HashMap::new();

        chars_count.insert(previous_char, 1);

        'inner: for c in password.chars() {
            if c < previous_char {
                //println!("skipping because {} is smaller than {}", c, previous_char);
                continue 'outer;
            }

            if chars_count.contains_key(&c) {
                chars_count.insert(c, chars_count[&c] + 1);
            } else {
                chars_count.insert(c, 1);
            }

            previous_char = c;
        }

        for (c, count) in chars_count {
            if count == 2 {
                satisfying_passwords_count = satisfying_passwords_count + 1;
                //println!("{} conforms", password);
                continue 'outer;
            }
        }

        //println!("{} does not conform", password);
    }

    println!("{}", satisfying_passwords_count);
}
