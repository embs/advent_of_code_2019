use std::fs;

fn tally_fuel(mass: i32) -> i32 {
    let fraction: f32 = mass as f32 / 3.0;
    let rounded = fraction.floor() as i32;

    if rounded <= 2 {
        0
    } else {
        let result = rounded - 2;

        result + tally_fuel(result)
    }
}

fn main() {
    let contents = fs::read_to_string("day_1.input")
	.expect("Something went wrong reading the file");

    let masses = contents.split("\n");
    for mass in masses {
        let mass_int: i32 = mass.parse().unwrap();
        println!("{}", tally_fuel(mass_int));
    }
}
