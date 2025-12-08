use std::collections::HashMap;
use crate::EmptyResult;
use std::fs;
use std::ops::Index;

#[test]
fn day_07_part1() -> EmptyResult {
    let mut lines = fs::read_to_string("input/day-07.txt")?
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut split = 0;
    for line in 0..lines.len() - 1 {
        println!("{:?}", lines[line]);
        for col in 0..lines[line].len() {
            let curr = lines[line][col];
            if curr == '|' || curr == 'S' {
                let next = lines[line + 1][col];
                if next == '^' {
                    split += 1;
                    lines[line + 1][col - 1] = '|';
                    lines[line + 1][col + 1] = '|';
                } else {
                    lines[line + 1][col] = '|';
                }
            }
        }
    }
    println!("{}", split);
    Ok(())
}

#[test]
fn day_07_part2() -> EmptyResult {
    let mut lines = fs::read_to_string("input/day-07.txt")?
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let col = lines.remove(0).iter().position(|c| *c == 'S').unwrap();

    let mut particles = HashMap::new();
    particles.insert(col, 1);

    for line in lines {
        let mut new_particles = HashMap::new();
        for (&col, &mut val) in particles.iter_mut() {
            match line[col] {
                '^' => {
                    *new_particles.entry(col - 1).or_default() += val;
                    *new_particles.entry(col + 1).or_default() += val;
                }
                '.' => *new_particles.entry(col).or_default() += val,
                _ => unreachable!(),
            }
        }
        particles = new_particles;
    }
    println!("{}", particles.values().sum::<u64>());
    Ok(())
}
