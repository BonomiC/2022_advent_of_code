use std::env;
use std::fs;
use std::cmp::min;
use std::cmp::max;

fn find_num_start(line: &str, start_idx: i32) -> (i32, i32, u64)
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

    if start == -1 { return (-1, -1, 0); }

    let mut end: usize = line.len();
    for i in (start as usize)..line.len()
    {
        if !line.chars().nth(i).unwrap().is_digit(10)
        {
            end = i;
            break;
        }
    }

    return (start, end as i32, line[(start as usize)..(end as usize)].parse::<u64>().unwrap());
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

fn find_gear(line: &str, start: i32) -> i32
{
    let f = line[(start as usize)..].find("*");
    if f.is_none() { return -1 }

    return f.unwrap() as i32 + start;
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines: Vec<&str> = contents.split("\n").collect();

    lines.truncate(lines.len()-1);

    let mut sum: u64 = 0;

    for (i, line) in lines.iter().enumerate()
    {
        println!("Line {i}");

        let mut g: i32 = 0;
        let (mut x, mut y, mut num): (i32, i32, u64) = (0, 0, 0);

        loop
        {
            let mut ratio: u64 = 1;
            let mut count: u32 = 0;
            
            g = find_gear(&line, g);
            if g == -1 { break; }

            (x, y, num) = (0, 0, 0);
            while x != -1
            {
                (x, y, num) = find_num_start(&line, y);
                if y == g
                {
                   ratio *= num; 
                   count += 1;
                   println!("{}", num);
                }
                if x == g+1
                {
                   ratio *= num; 
                   count += 1;
                   println!("{}", num);
                }
            }

            (x, y, num) = (0, 0, 0);
            if i < lines.len() - 1
            {
                loop
                {
                    (x, y, num) = find_num_start(&lines[i+1], y);
                    if x == -1 { break; }

                    if g >= x-1 && g <= y
                    {
                        ratio *= num;
                        count += 1;
                    }
                }
            }

            (x, y, num) = (0, 0, 0);
            if i > 0
            {
                loop
                {
                    (x, y, num) = find_num_start(&lines[i-1], y);
                    if x == -1 { break; }

                    if g >= x-1 && g <= y
                    {
                        ratio *= num;
                        count += 1;
                    }
                }
            }

            if count == 2
            {
                sum += ratio;
                println!("Line {} is gear", i+1);
            }

            g += 1
        }
    }

    println!("Sum {sum}");
}
