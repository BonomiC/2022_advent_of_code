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

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut sum: u32 = 0;

    let mut game_id: u32 = 0;

    for mut line in lines
    {
        game_id += 1;
        let colon_idx = line.find(":").unwrap();
        line = &line[colon_idx+2..]; // Remove "Game X: " from start of line

        let hands: Vec<&str> = line.split("; ").collect();

        // dbg!(&hands);

        let mut possible = true;

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

                println!("Game id: {game_id}");
                *map.get_mut(key).unwrap() += num.parse::<u32>().unwrap();

                hand = &hand[0..comma_idx];
            }

            if map["red"] > MAX_RED { possible = false; }
            if map["green"] > MAX_GREEN { possible = false; }
            if map["blue"] > MAX_BLUE { possible = false; }
        }

        if possible
        {
            sum += game_id;
        }
    }

    println!("Sum: {sum}");
}
