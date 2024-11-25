use std::env;
use std::fs;
use std::cmp::min;
use std::cmp::max;

fn find_num_start(line: &str, start_idx: i32) -> (i32, i32)
{
    let mut start: i32 = -1;
    for i in (start_idx as usize)..line.len()
    {
        if line.chars().nth(i).unwrap().is_digit(10)
        {
            start = i as i32;
            break;
        }
    }

    if start == -1 { return (-1, -1); }

    let mut end: usize = line.len();
    for i in (start as usize)..line.len()
    {
        if !line.chars().nth(i).unwrap().is_digit(10)
        {
            end = i;
            break;
        }
    }

    return (start, end as i32);
}

fn range_has_symbol(line: &str, start_idx: i32, end_idx: i32) -> bool
{
    for i in (max(start_idx, 0) as usize)..(min(end_idx, line.len() as i32) as usize)
    {
        let c: char = line.chars().nth(i).unwrap();
        if !c.is_digit(10) && c != '.'
        {
            return true;
        }
    }

    return false;
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines: Vec<&str> = contents.split("\n").collect();

    lines.truncate(lines.len()-1);

    let mut sum: u32 = 0;

    for (i, line) in lines.iter().enumerate()
    {
        let (mut x, mut y): (i32, i32) = (0, 0);

        loop
        {
            (x, y) = find_num_start(&line, y);
            if x == -1 { break; }

            let mut is_part = false;
            
            // Search same line
            is_part |= range_has_symbol(&line, x-1, y+1);
            // Search line below
            if i < line.len()-1
            {
                is_part |= range_has_symbol(&lines[i+1], x-1, y+1);
            }
            // Search line above
            if i > 0
            {
                is_part |= range_has_symbol(&lines[i-1], x-1, y+1);
            }

            if is_part
            {
                let num = &line[(x as usize)..(y as usize)].parse::<u32>().unwrap();
                sum += num;
            }
        }
    }

    println!("Sum {sum}");
}
