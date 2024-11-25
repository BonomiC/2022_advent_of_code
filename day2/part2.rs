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

    for mut line in lines
    {
        let mut min_cubes: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        let colon_idx = line.find(":").unwrap();
        line = &line[colon_idx+2..]; // Remove "Game X: " from start of line

        let hands: Vec<&str> = line.split("; ").collect();

        for mut hand in hands
        {
            let mut map = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0)
            ]);

            let mut comma_idx: usize = usize::MAX;

            while comma_idx > 0
            {
                // println!("{}", hand);
                let space_idx: usize = hand.rfind(" ").unwrap();
                comma_idx = hand.rfind(",").unwrap_or(0);

                let key = &hand[space_idx+1..];
                let num = &hand[comma_idx..space_idx].replace(", ", "");

                *map.get_mut(key).unwrap() += num.parse::<u32>().unwrap();

                hand = &hand[0..comma_idx];
            }

            for key in map.keys()
            {
                if map[key] > min_cubes[key]
                {
                    *min_cubes.entry(key).or_insert(0) = map[key];
                }
            }
        }

        let mut power = min_cubes["red"];
        power *= min_cubes["green"];
        power *= min_cubes["blue"];

        sum += power;
    }

    println!("Sum {sum}");
}
