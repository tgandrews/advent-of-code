use std::{error::Error, fs::File, io::Read};

fn str_to_num(str: &str) -> i32 {
    match str.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!("Cannot parse: {}", str),
    }
}

fn main() {
    let mut f = match File::open("data") {
        Err(why) => panic!("Failed to open: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut data = String::new();
    match f.read_to_string(&mut data) {
        Err(why) => panic!("Failed to read: {}", Error::description(&why)),
        Ok(_) => println!("Sucessfully read data"),
    };

    let lines = data.lines().map(str_to_num);

    let mut previous = 0;
    let mut count = 0;
    for line in lines.into_iter() {
        if line > previous && previous > 0 {
            count += 1;
        }
        previous = line;
    }
    println!("Increases: {}", count)
}
