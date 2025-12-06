use crate::EmptyResult;
use std::error::Error;
use std::fs;

#[test]
fn day_06_part1() -> EmptyResult {
    let mut input = read_lines()?;

    let mut reducers: Vec<Reducer> = input
        .pop()
        .unwrap()
        .iter()
        .map(|op| match op.as_str() {
            "+" => Reducer::new(Box::new(|a, b| a + b), 0),
            "*" => Reducer::new(Box::new(|a, b| a * b), 1),
            _ => unreachable!(),
        })
        .collect();

    for line in input {
        for i in 0..line.len() {
            let value = line[i].parse::<i64>()?;
            reducers[i].apply(value);
        }
    }

    let result: i64 = reducers.iter().map(|r| r.value).sum();
    println!("{}", result);
    Ok(())
}

#[test]
fn day_06_part2() -> EmptyResult {
    let mut lines = fs::read_to_string("input/day-06.txt")?
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut ops = lines
        .pop()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|op| match op {
            "+" => Reducer::new(Box::new(|a, b| a + b), 0),
            "*" => Reducer::new(Box::new(|a, b| a * b), 1),
            _ => unreachable!(),
        })
        .collect::<Vec<Reducer>>();

    let mut column = 0;

    let cols = lines.iter().map(|r| r.len()).max().unwrap();

    for col in 0..cols {
        let mut num = 0i64;
        for row in 0..lines.len() {
            let char = lines[row].chars().nth(col);
            let Some(char) = char else {
                continue;
            };
            if char == ' ' {
                continue;
            }
            let val = char as i8 - '0' as i8;
            num *= 10;
            num += val as i64;
        }

        if num == 0 {
            column += 1;
        } else {
            ops[column].apply(num);
        }
    }

    let result: i64 = ops.iter().map(|r| r.value).sum();
    println!("{}", result);
    Ok(())
}

struct Reducer {
    func: Box<dyn Fn(i64, i64) -> i64>,
    value: i64,
}

impl Reducer {
    fn new(func: Box<dyn Fn(i64, i64) -> i64>, value: i64) -> Reducer {
        Self { func, value }
    }

    fn apply(&mut self, value: i64) {
        self.value = (self.func)(self.value, value);
    }
}

fn read_lines() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    Ok(fs::read_to_string("input/day-06.txt")?
        .lines()
        .map(|l| {
            l.split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>())
}
