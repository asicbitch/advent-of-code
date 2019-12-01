use std::io;

fn u32_from_stdin() -> Vec<u32> {
    let mut inputs: Vec<u32> = Vec::new();

    loop {
        let mut temp_string = String::new();
        io::stdin()
            .read_line(&mut temp_string)
            .expect("Failed to read line.");

        if temp_string.is_empty() {
            break;
        } else {
            let temp_value: u32 = match temp_string.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            inputs.push(temp_value);
        }
    }

    return inputs;
}

fn calc_fuel(m: u32) -> u32 {
    let temp = (m as f32) / 3.0;
    (temp as u32) - 2
}

fn main() {
    println!("Input module masses, and an empty line when done.");
    let inputs: Vec<u32> = u32_from_stdin();
    let mut sum: u32 = 0;
    for val in inputs {
        sum = sum + calc_fuel(val);
    }
    println!("Total fuel mass needed: {}", sum);
}
