use std::env;
use std::fs;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines: Vec<&str> = contents.split("\n").collect();

    lines.truncate(lines.len()-1);

    let mut sum: u32 = 0;

    for line in lines
    {
        let chars: Vec<char> = line.chars().collect();

        let mut reading = String::from("");

        for i in 0..chars.len()
        {
            if chars[i].is_digit(10)
            {
                reading.push(chars[i]);
                break;
            }
        }
        for i in (0..chars.len()).rev()
        {
            if chars[i].is_digit(10)
            {
                reading.push(chars[i]);
                break;
            }
        }

        sum += reading.parse::<u32>().unwrap();
    }

    println!("Sum: {sum}");
}
