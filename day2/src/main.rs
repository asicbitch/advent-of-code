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

fn main() {
    let mut program = u32_from_stdin();

    // Initialize program to state 1202 Program Alarm
    program[1] = 12;
    program[2] = 2;

    for i in (0..program.len()).step_by(4) {
        let instruction: Vec<_> = program[i..i + 4].iter().cloned().collect();
        match instruction[0] {
            1 => {
                program[instruction[3] as usize] =
                    program[instruction[1] as usize] + program[instruction[2] as usize]
            }
            2 => {
                program[instruction[3] as usize] =
                    program[instruction[1] as usize] * program[instruction[2] as usize]
            }
            99 => break,
            _ => println!("Invalid op code!"),
        }
    }

    println!("Value at address 0: {}", program[0]);
}
