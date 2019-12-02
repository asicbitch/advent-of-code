use std::io;

fn u32_from_stdin() -> Vec<u32> {
    let mut inputs: Vec<u32> = Vec::new();

    let mut temp_string = String::new();
    io::stdin()
        .read_line(&mut temp_string)
        .expect("Failed to read line.");

    for val in temp_string.split(',') {
        let temp_value: u32 = match val.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        inputs.push(temp_value);
    }

    return inputs;
}

fn run_program(prog: &mut Vec<u32>) -> u32 {
    for i in (0..prog.len()).step_by(4) {
        let instruction: Vec<_> = prog[i..i + 4].iter().cloned().collect();
        match instruction[0] {
            1 => {
                prog[instruction[3] as usize] =
                    prog[instruction[1] as usize] + prog[instruction[2] as usize]
            }
            2 => {
                prog[instruction[3] as usize] =
                    prog[instruction[1] as usize] * prog[instruction[2] as usize]
            }
            99 => break,
            _ => println!("Invalid op code!"),
        }
    }
    prog[0]
}

fn main() {
    let initial_program = u32_from_stdin();

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut running_program = initial_program.clone();
            running_program[1] = noun;
            running_program[2] = verb;
            if run_program(&mut running_program) == 19690720 {
                println!("Noun and verb: {}, {}", noun, verb);
                break 'outer;
            }
        }
    }
}
