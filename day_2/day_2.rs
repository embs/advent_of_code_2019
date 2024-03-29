fn process(noun: usize, verb: usize) {
    let mut program = [1,noun,verb,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,5,23,2,9,23,27,1,5,27,31,1,5,31,35,1,35,13,39,1,39,9,43,1,5,43,47,1,47,6,51,1,51,13,55,1,55,9,59,1,59,13,63,2,63,13,67,1,67,10,71,1,71,6,75,2,10,75,79,2,10,79,83,1,5,83,87,2,6,87,91,1,91,6,95,1,95,13,99,2,99,13,103,1,103,9,107,1,10,107,111,2,111,13,115,1,10,115,119,1,10,119,123,2,13,123,127,2,6,127,131,1,13,131,135,1,135,2,139,1,139,6,0,99,2,0,14,0];
    let mut program_counter = 0;

    loop {
        println!("program_counter: {}", program_counter);
        let operation = program[program_counter];
        if operation == 99 {
            println!("op 99");
            //for i in program.iter() {
            //    print!("{},", i);
            //}
            //println!("");
            //std::process::exit(0);
            if program[0] == 19690720 {
                println!("noun: {}", noun);
                println!("verb: {}", verb);
                std::process::exit(0);
            }
            return;
        }
        let first_operand_position = program[program_counter + 1];
        let second_operand_position = program[program_counter + 2];
        let result_position = program[program_counter + 3];

        if operation == 1 {
            println!("op 1");
            println!("1st pos: {}", first_operand_position);
            println!("1st val: {}", program[first_operand_position]);
            println!("2nd pos: {}", second_operand_position);
            println!("2nd val: {}", program[second_operand_position]);
            program[result_position] = program[first_operand_position] + program[second_operand_position];
        } else if operation == 2 {
            println!("op 2");
            program[result_position] = program[first_operand_position] * program[second_operand_position];
        } else {
            panic!("Unknown operation: {}", operation);
        }

        program_counter = program_counter + 4;
    }
}

fn main() {
    //let mut program = [1, 0, 0, 0, 99];
    //let mut program = [2,3,0,3,99];
    //let mut program = [2,4,4,5,99,0];
    //let mut program = [1,1,1,4,99,5,6,0,99];

    for noun in 0..=99 {
        for verb in 0..=99 {
            process(noun, verb);
        }
    }
}
