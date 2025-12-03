use crate::EmptyResult;
use std::fs;

#[test]
fn day_01_part1() -> EmptyResult {
    let turns: Vec<i32> = fs::read_to_string("input/day-01.txt")?
        .lines()
        .into_iter()
        .map(parse_line)
        .collect();

    let mut dial = 50;
    let mut result = 0;

    for turn in turns {
        dial += turn;
        dial %= 100;
        if dial < 0 {
            dial = 100 + dial;
        }

        if dial == 0 {
            result += 1;
        }
    }

    println!("{}", result);
    Ok(())
}

#[test]
fn day_01_part2() -> EmptyResult {
    let turns: Vec<i32> = fs::read_to_string("input/day-01.txt")?
        .lines()
        .into_iter()
        .map(parse_line)
        .collect();

    let mut dial = 50;
    let mut result = 0;

    for turn in turns {
        let start = dial;

        result += turn.abs() / 100;
        dial += turn % 100;

        if dial > 100 {
            result += dial / 100;
        }

        dial %= 100;

        if dial < 0 {
            if start != 0 {
                result += 1;
            }
            dial += 100;
        }

        if dial == 0 {
            result += 1;
        }
    }

    println!("{}", result);
    Ok(())
}

fn parse_line(line: &str) -> i32 {
    let sign = match line.as_bytes()[0] {
        b'R' => 1,
        b'L' => -1,
        _ => panic!(),
    };
    let num = line[1..].parse::<i32>().unwrap();

    sign * num
}
