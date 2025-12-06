use crate::EmptyResult;
use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;

#[test]
fn day_05_part1() -> EmptyResult {
    let (fresh, ids) = parse()?;

    let result = ids
        .iter()
        .filter(|id| fresh.iter().any(|f| f.contains(id)))
        .count();

    println!("{}", result);

    Ok(())
}

#[test]
fn day_05_part2() -> EmptyResult {
    let (mut fresh, _) = parse()?;

    fresh.sort_by_key(|r| *r.start());

    let mut ranges = Vec::new();
    let mut current = fresh.remove(0);

    for range in fresh {
        match merge(current, range) {
            MergeResult::One(one) => current = one,
            MergeResult::Two(one, two) => {
                ranges.push(one);
                current = two;
            }
        }
    }

    ranges.push(current);

    let result: i64 = ranges.into_iter().map(|r| r.count() as i64).sum();
    println!("{}", result);

    Ok(())
}

fn parse() -> Result<(Vec<RangeInclusive<i64>>, Vec<i64>), Box<dyn Error>> {
    let mut fresh = Vec::new();
    let mut ids = Vec::new();
    let mut first_block = true;

    for line in fs::read_to_string("input/day-05.txt")?.lines() {
        if line.is_empty() {
            first_block = false;
        } else if first_block {
            let pair = line.split("-").collect::<Vec<&str>>();
            let first: i64 = pair[0].parse()?;
            let second: i64 = pair[1].parse()?;
            fresh.push(first..=second);
        } else {
            let id: i64 = line.parse()?;
            ids.push(id);
        }
    }
    Ok((fresh, ids))
}

fn merge(first: RangeInclusive<i64>, second: RangeInclusive<i64>) -> MergeResult<RangeInclusive<i64>> {
    if first.contains(second.start())
        || first.contains(second.end())
        || second.contains(first.start())
        || second.contains(first.end())
    {
        MergeResult::One(*first.start().min(second.start())..=*first.end().max(second.end()))
    } else {
        MergeResult::Two(first, second)
    }
}

enum MergeResult<T> {
    One(T),
    Two(T, T),
}
