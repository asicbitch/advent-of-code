use std::io;

fn i32_from_stdin() -> Vec<i32> {
    let mut inputs: Vec<i32> = Vec::new();

    loop {
        let mut temp_string = String::new();
        io::stdin()
            .read_line(&mut temp_string)
            .expect("Failed to read line.");

        if temp_string.is_empty() {
            break;
        } else {
            let temp_value: i32 = match temp_string.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            inputs.push(temp_value);
        }
    }

    return inputs;
}

fn calc_fuel(m: i32) -> i32 {
    let temp = (m as f32) / 3.0;
    let mut res = (temp as i32) - 2;

    if res > 0 {
        res = res + calc_fuel(res);
    } else {
        res = 0;
    }

    return res;
}

fn main() {
    println!("Input module masses, and an empty line when done.");
    let inputs: Vec<i32> = i32_from_stdin();
    let mut sum: i32 = 0;

    for val in inputs {
        sum = sum + calc_fuel(val);
    }

    println!("Total fuel mass needed: {}", sum);
}
