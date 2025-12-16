use crate::EmptyResult;
use std::fs;

#[test]
fn day_12_part1() -> EmptyResult {
    let input: Vec<_> = fs::read_to_string("input/day-12.txt")?
        .split("\n\n")
        .map(String::from)
        .collect();
    let shapes = input[0..6]
        .iter()
        .map(|line| line.chars().filter(|&c| c == '#').count() as i64)
        .collect::<Vec<i64>>();

    let trees = input[6]
        .split("\n")
        .map(parse_tree)
        .collect::<Vec<_>>();

    let result = trees.iter()
        .filter(|(width, height, counts)| {
            let mut area = 0;
            for i in 0..counts.len() {
                area += shapes[i] * counts[i]
            }
            area <= width * height
        }).count();

    println!("{result}");

    Ok(())
}

fn parse_tree(input: &str) -> (i64, i64, Vec<i64>) {
    let (left, right) = input.split_once(": ").unwrap();
    let (width, height) = left.split_once("x").unwrap();
    let counts = right
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    (width.parse().unwrap(), height.parse().unwrap(), counts)
}
