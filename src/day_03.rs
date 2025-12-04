use crate::EmptyResult;
use std::fs;

#[test]
fn day_03_part1() -> EmptyResult {
    let result = fs::read_to_string("input/day-03.txt")?
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x as i32 - '0' as i32)
                .collect::<Vec<i32>>()
        })
        .map(max)
        .sum::<i32>();
    println!("{:?}", result);
    Ok(())
}

#[test]
fn day_03_part2() -> EmptyResult {
    let result = fs::read_to_string("input/day-03.txt")?
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x as i32 - '0' as i32)
                .collect::<Vec<i32>>()
        })
        .map(part2_max)
        .sum::<i64>();
    println!("{:?}", result);
    Ok(())
}

fn max(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..nums.len() {
        if nums[i] * 10 <= max {
            continue;
        }
        for j in i + 1..nums.len() {
            max = max.max(nums[i] * 10 + nums[j]);
        }
    }
    max
}

fn part2_max(nums: Vec<i32>) -> i64 {
    let mut digits_left = 12;
    let mut start = 0;
    let mut value = 0i64;

    while digits_left > 0 {
        let slice_to_check = &nums.as_slice()[start..nums.len() - (digits_left - 1)];
        let idx = first_max_idx(slice_to_check);
        value *= 10;
        value += nums[start + idx] as i64;
        start += idx + 1;
        digits_left -= 1;
    }

    value
}

fn first_max_idx(nums: &[i32]) -> usize {
    let mut max = 0;
    let mut ret = 0;

    for i in 0..nums.len() {
        let curr = nums[i];
        if curr > max {
            max = curr;
            ret = i;
        }
    }

    ret
}