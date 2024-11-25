use std::env;
use std::fs;
use std::collections::HashMap;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines: Vec<&str> = contents.split("\n").collect();

    lines.truncate(lines.len()-1);

    let mut sum: u32 = 0;

    let number_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    for line in lines
    {
        let chars: Vec<char> = line.chars().collect();

        let mut reading = String::from("");

        let mut digit1_idx: usize = usize::MAX;
        let mut digit2_idx: i32 = -1;

        let mut num1: i32 = 0;
        let mut num1_idx: usize = usize::MAX;

        let mut num2: i32 = 0;
        let mut num2_idx: i32 = -1;

        for key in number_map.keys()
        {
            let i = line.find(key);
            if i.is_some() && i.unwrap() < num1_idx
            {
                num1 = number_map[key];
                num1_idx = i.unwrap();
            }

            let i2 = line.rfind(key);
            if i2.is_some() && (i2.unwrap() as i32) > num2_idx
            {
                num2 = number_map[key];
                num2_idx = i2.unwrap() as i32;
            }
        }

        for i in 0..chars.len()
        {
            if chars[i].is_digit(10)
            {
                // reading.push(chars[i]);
                digit1_idx = i;
                break;
            }
        }

        for i in (0..chars.len()).rev()
        {
            if chars[i].is_digit(10)
            {
                // reading.push(chars[i]);
                digit2_idx = i as i32;
                break;
            }
        }

        if digit1_idx < num1_idx
        {
            reading.push(chars[digit1_idx]);
        }
        else
        {
            reading.push_str(&num1.to_string());
        }

        if digit2_idx > num2_idx
        {
            reading.push(chars[digit2_idx as usize]);
        }
        else
        {
            reading.push_str(&num2.to_string());
        }

        sum += reading.parse::<u32>().unwrap();
    }

    println!("Sum: {sum}");
}
