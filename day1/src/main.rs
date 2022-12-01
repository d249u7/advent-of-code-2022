use std::{collections::HashMap, fs};

fn main() {
    let input =
        fs::read_to_string("/home/dragut/Projects/advent-of-code-2022/day1/inputs/example.txt")
            .expect("The file could not be read");

    let lines = input.lines().collect::<Vec<&str>>();
    let mut calories_by_elf: HashMap<i64, i64> = HashMap::new();
    let mut counter = 1;

    for line in lines {
        let calories = line.parse::<i64>();

        match calories {
            Ok(_) => {
                let cals = calories.unwrap();
                calories_by_elf
                    .entry(counter)
                    .and_modify(|e| *e += cals)
                    .or_insert(cals);
            }
            Err(_) => counter += 1,
        };
    }
    let mut maxes = (&0, &0, &0);

    for (_, v) in calories_by_elf.iter() {
        if v >= maxes.0 {
            maxes.0 = v;
        } else if v >= maxes.1 {
            maxes.1 = v;
        } else if v >= maxes.2 {
            maxes.2 = v;
        }

        println!("{:#?}", maxes);
    }

    let res = maxes.0 + maxes.1 + maxes.2;

    println!("{:?}\n {} {:?}", calories_by_elf, res, maxes);
}
