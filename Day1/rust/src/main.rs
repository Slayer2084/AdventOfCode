use std::{fs};

fn main() {
    let contents = fs::read_to_string("C:/Users/Paul/PycharmProjects/AdventOfCode/Day1/data.txt")
        .expect("Wrong file path!");
    let data: Vec<_> = contents.lines().collect();

    let mut result1: Vec<i32> = Vec::new();
    let mut sum_of_elve = 0;

    for line in data.iter() {
        match line {
           &"" => {
            result1.push(sum_of_elve);
            sum_of_elve = 0;
           },
           _ => {
            let line_as_int: i32 = line.parse().unwrap();
            sum_of_elve = sum_of_elve + line_as_int;
           },
        }
    }
    let max_value = result1.iter().max();
    match max_value {
        Some(max) => println!( "Max value: {}", max),
        None      => println!( "Vector is empty" ),
    }

    
}
