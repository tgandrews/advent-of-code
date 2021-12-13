use std::{fs::File, io::Read};

fn str_to_num(str: &str) -> i32 {
    match str.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!("Cannot parse: {}", str),
    }
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

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in data.lines() {
        let mut iter = line.split_ascii_whitespace();
        let action = iter.next().unwrap();
        let distance = str_to_num(iter.next().unwrap());

        match action {
            "forward" => {
                horizontal = horizontal + distance;
                depth = depth + (aim * distance);
            }
            "down" => aim = aim + distance,
            "up" => aim = aim - distance,
            _ => panic!("Unknown action: {}", action),
        }
    }

    let result = horizontal * depth;
    println!("H:{} x D:{} = {}", horizontal, depth, result)
}
