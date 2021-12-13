use std::{fs::File, io::Read};

fn most_common_char<'a>(data: &'a Vec<&str>) -> (&'a str, &'a str) {
    let mut one_count = 0;
    let mut zero_count = 0;
    for char in data {
        match char {
            &"1" => one_count = one_count + 1,
            &"0" => zero_count = zero_count + 1,
            _ => panic!("Unexpected char: \"{}\"", char),
        };
    }
    if one_count > zero_count {
        ("1", "0")
    } else {
        ("0", "1")
    }
}

fn binary_char_array_to_number(data: Vec<&str>) -> isize {
    let binary_string = data.join("");
    isize::from_str_radix(&binary_string, 2).unwrap()
}

fn main() {
    let mut f = match File::open("data") {
        Err(why) => panic!("Failed to open: {}", why),
        Ok(file) => file,
    };

    let mut data = String::new();
    match f.read_to_string(&mut data) {
        Err(why) => panic!("Failed to read: {}", why),
        Ok(_) => println!("Sucessfully read data"),
    };

    let lines = data.lines();

    let mut data: Vec<Vec<&str>> = vec![vec!["0"; 1000]; 12];

    let lines_of_bits = lines.map(|line| line.trim().split("").filter(|s| !s.is_empty()));

    for (line_index, line) in lines_of_bits.enumerate() {
        for (char_index, char) in line.enumerate() {
            data[char_index][line_index] = char
        }
    }

    let mut gamma_rate: Vec<&str> = vec![];
    let mut epsilon_rate: Vec<&str> = vec![];

    for column_index in 0..data.len() {
        let column_data = data.get(column_index).unwrap();
        let (most_common, least_common) = most_common_char(column_data);
        gamma_rate.push(most_common);
        epsilon_rate.push(least_common);
    }

    let gamma_value = binary_char_array_to_number(gamma_rate);
    let epsilon_value = binary_char_array_to_number(epsilon_rate);

    println!(
        "Gamma: {} x Epsilon: {}. Result = {}",
        gamma_value,
        epsilon_value,
        gamma_value * epsilon_value
    )
}
