use crate::EmptyResult;
use std::fs;
use std::ops::RangeInclusive;

#[test]
fn day_02_part1() -> EmptyResult {
    let input: i64 = fs::read_to_string("input/day-02.txt")?
        .split(",")
        .map(parse)
        .flat_map(|range| range.filter(is_invalid))
        .sum();
    println!("{:?}", input);
    Ok(())
}

#[test]
fn day_02_part2() -> EmptyResult {
    let input: i64 = fs::read_to_string("input/day-02.txt")?
        .split(",")
        .map(parse)
        .flat_map(|range| range.filter(is_invalid_2))
        .sum();
    println!("{:?}", input);
    Ok(())
}

fn parse(input: &str) -> RangeInclusive<i64> {
    let mut split = input.split("-");
    let (first, second) = (
        split.next().unwrap().parse::<i64>().unwrap(),
        split.next().unwrap().parse::<i64>().unwrap(),
    );
    first..=second
}

fn is_invalid(num: &i64) -> bool {
    let str = num.to_string();
    if str.len() % 2 == 1 {
        return false;
    }

    let first = &str[0..str.len() / 2];
    let second = &str[str.len() / 2..];

    first == second
}

fn is_invalid_2(num: &i64) -> bool {
    let str = num.to_string();

    for i in 1..=str.len() / 2 {
        let Some(chunks) = chunk(&str, i) else {
            continue;
        };
        let first = chunks[0];
        if chunks.iter().all(|&x| x == first) {
            return true;
        }
    }

    false
}

fn chunk(str: &str, size: usize) -> Option<Vec<&str>> {
    if str.len() % size != 0 {
        return None;
    }

    let mut ret = Vec::new();

    let mut start = 0usize;
    let mut end = size;

    while end <= str.len() {
        ret.push(&str[start..end]);
        start = end;
        end += size;
    }

    Some(ret)
}

#[test]
fn test_is_invalid() {
    assert!(is_invalid(&11));
    assert!(is_invalid(&22));
    assert!(!is_invalid(&10));

    assert!(is_invalid_2(&121212));
    assert!(is_invalid_2(&22));
}

#[test]
fn test_chunk() {
    println!("{:?}", chunk("abcdef", 2));
    println!("{:?}", chunk("abcdef", 3));
    println!("{:?}", chunk("abcdef", 4));
    println!("{:?}", chunk("abcdefg", 3));
}