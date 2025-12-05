use crate::EmptyResult;
use std::cmp::{max, min};
use std::fs;

#[test]
fn day_04_part1() -> EmptyResult {
    let input = fs::read_to_string("input/day-04.txt")?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid { data: input };

    println!("{}", to_remove(&grid).len());

    Ok(())
}

#[test]
fn day_04_part2() -> EmptyResult {
    let input = fs::read_to_string("input/day-04.txt")?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut grid = Grid { data: input };

    let mut total = 0;
    loop {
        let remove = to_remove(&grid);
        if remove.len() == 0 {
            break;
        }
        total += remove.len();
        for (x, y) in remove {
            grid.set(x, y, '.');
        }
    }

    println!("{}", total);

    Ok(())
}

fn to_remove(grid: &Grid<char>) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    for x in 0..grid.x_len() {
        for y in 0..grid.y_len() {
            if *grid.value(x, y) == '.' {
                continue;
            }
            if grid.surrounding(x, y).iter().filter(|&&x| x == '@').count() < 4 {
                ret.push((x, y));
            }
        }
    }

    ret
}

struct Grid<T: Clone> {
    data: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    fn y_len(&self) -> usize {
        self.data.len()
    }

    fn x_len(&self) -> usize {
        self.data[0].len()
    }

    fn value(&self, x: usize, y: usize) -> &T {
        &self.data[y][x]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y][x] = value;
    }

    fn surrounding(&self, x: usize, y: usize) -> Vec<T> {
        let y_min = y.saturating_sub(1);
        let y_max = min(self.data.len() - 1, y + 1);
        let x_min = x.saturating_sub(1);
        let x_max = min(self.data[0].len() - 1, x + 1);

        let mut ret = Vec::new();

        for y_idx in y_min..=y_max {
            for x_idx in x_min..=x_max {
                if y == y_idx && x == x_idx {
                    continue;
                } else {
                    ret.push(self.data[y_idx][x_idx].clone());
                }
            }
        }

        ret
    }
}
