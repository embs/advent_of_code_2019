fn main() {
    let mut satisfying_passwords_count = 0;

    'outer: for p in 284639..=748759 {
    //'outer: for p in 284639..=300000 {
        let password = p.to_string();

        // It is a six-digit number.
        if password.len() != 6 { continue; }

        let mut previous_char = 'a';
        let mut adjacent_same_digits = false;

        'inner: for c in password.chars() {
            if previous_char == 'a' {
                previous_char = c;
                continue;
            }

            if c < previous_char {
                //println!("skipping because {} is smaller than {}", c, previous_char);
                continue 'outer;
            }

            if c == previous_char {
                adjacent_same_digits = true;
                //println!("{} is the same as {}", c, previous_char);
            }

            previous_char = c;
        }

        if adjacent_same_digits {
            satisfying_passwords_count = satisfying_passwords_count + 1;
            //println!("{} conforms", password);
        }
    }

    println!("{}", satisfying_passwords_count);
}
