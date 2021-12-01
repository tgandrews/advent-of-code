use std::{error::Error, fs::File, io::Read};

fn str_to_num(str: &str) -> i32 {
    match str.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!("Cannot parse: {}", str),
    }
}

fn set_value(list: &mut Vec<i32>, index: i32, value: i32) {
    if index < 0 {
        return;
    }
    let idx = index as usize;
    if idx >= list.len() {
        return;
    }
    list[idx] = list[idx] + value;
    return;
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

    let lines: Vec<i32> = data.lines().map(str_to_num).collect();
    println!("Lines len: {}", lines.len());
    let mut windows: Vec<i32> = vec![0; lines.len() - 2];
    println!("Windows len: {}", windows.len());
    // let mut count = 0;
    for index in 0..lines.len() {
        let line = lines[index];

        let first_window_idx = (index as i32) - 2;
        set_value(&mut windows, first_window_idx, line);
        set_value(&mut windows, first_window_idx + 1, line);
        set_value(&mut windows, first_window_idx + 2, line);
    }

    let mut previous = 0;
    let mut count = 0;
    for window in windows {
        if window > previous && previous > 0 {
            count += 1;
        }
        previous = window;
    }
    println!("Increases: {}", count);
}
