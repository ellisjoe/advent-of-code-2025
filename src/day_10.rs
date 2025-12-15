use crate::EmptyResult;
use std::fs;

#[test]
fn day_10_part1() -> EmptyResult {
    let machines = fs::read_to_string("input/day-10.txt")?
        .lines()
        .map(Machine::parse)
        .collect::<Vec<_>>();
    let result: i64 = machines.iter().map(|m| m.min_presses()).sum();
    println!("{result:?}");
    Ok(())
}

#[test]
fn day_10_part2() -> EmptyResult {
    let machines = fs::read_to_string("input/day-10.txt")?
        .lines()
        .map(Machine::parse)
        .collect::<Vec<_>>();
    let values = machines
        .iter()
        .map(|m| m.min_joltage())
        .map(|m| {
            println!("{m}");
            m
        }).collect::<Vec<_>>();

    println!("{values:?}");
    println!("{:?}", values.iter().sum::<i64>());

    Ok(())
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Vec<i32>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let parts = line.split(" ").collect::<Vec<_>>();
        let lights_input = parts[0];
        let lights = lights_input[1..lights_input.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();

        let buttons = parts[1..parts.len() - 1]
            .iter()
            .map(|input| Button::parse(input))
            .collect();

        let joltage_input = parts[parts.len() - 1];
        let joltage = joltage_input[1..joltage_input.len() - 1]
            .split(",")
            .map(|input| input.parse::<i32>().unwrap())
            .collect();

        Self {
            lights,
            buttons,
            joltage,
        }
    }

    fn min_presses(&self) -> i64 {
        let size = self.buttons.len();
        let mut bit_vec = BitVec::new(size);
        let mut min = i64::MAX;

        loop {
            let mut lights = init_vec(self.lights.len());
            for i in 0..bit_vec.bits.len() {
                if bit_vec.bits[i] {
                    self.buttons[i].apply(&mut lights);
                }
            }
            if lights == self.lights {
                min = min.min(bit_vec.bits_set())
            }
            if bit_vec.bits_set() == size as i64 {
                break;
            } else {
                bit_vec.next();
            }
        }
        min
    }

    fn presses(&self) -> Vec<BitVec> {
        let size = self.buttons.len();
        let mut bit_vec = BitVec::new(size);
        let mut ret = Vec::new();

        loop {
            let mut lights = init_vec(self.lights.len());
            for i in 0..bit_vec.bits.len() {
                if bit_vec.bits[i] {
                    self.buttons[i].apply(&mut lights);
                }
            }
            if lights == self.lights {
                ret.push(bit_vec.clone());
            }
            if bit_vec.bits_set() == size as i64 {
                break;
            } else {
                bit_vec.next();
            }
        }
        ret
    }

    fn min_joltage(&self) -> i64 {
        if self.joltage.iter().all(|&x| x == 0) {
            return 0;
        }
        let lights = to_lights(&self.joltage);
        let presses = Machine {
            lights,
            buttons: self.buttons.clone(),
            joltage: vec![],
        }
        .presses();

        let mut min = i64::MAX;

        for press in presses {
            let mut new_joltage = self.joltage.clone();
            for i in 0..press.bits.len() {
                if press.bits[i] {
                    self.buttons[i].sub(&mut new_joltage);
                }
            }
            if new_joltage.iter().any(|&x| x < 0) {
                continue
            }

            let new_joltage = new_joltage.iter().map(|x| x / 2).collect();
            let more_presses = Machine {
                lights: vec![],
                buttons: self.buttons.clone(),
                joltage: new_joltage,
            }
            .min_joltage();
            if more_presses == i64::MAX {
                continue
            } else {
                let count = press.bits_set() + 2 * more_presses;
                min = min.min(count);
            }
        }
        min
    }
}

fn to_lights(joltage: &Vec<i32>) -> Vec<bool> {
    joltage.iter().map(|&i| i % 2 == 1).collect()
}

#[derive(Debug, Clone)]
struct Button {
    lights: Vec<usize>,
}

impl Button {
    fn parse(input: &str) -> Self {
        let lights = input[1..input.len() - 1]
            .split(",")
            .map(|l| l.parse().unwrap())
            .collect::<Vec<usize>>();
        Self { lights }
    }

    fn apply(&self, lights: &mut Vec<bool>) {
        for light in &self.lights {
            lights[*light] = !lights[*light];
        }
    }

    fn sub(&self, values: &mut Vec<i32>) {
        for light in &self.lights {
            values[*light] -= 1;
        }
    }
}

#[derive(Clone)]
struct BitVec {
    bits: Vec<bool>,
}

impl BitVec {
    fn new(size: usize) -> Self {
        Self {
            bits: init_vec(size),
        }
    }

    fn bits_set(&self) -> i64 {
        self.bits.iter().filter(|&b| *b).count() as i64
    }

    fn next(&mut self) {
        for i in 0..self.bits.len() {
            if self.bits[i] {
                self.bits[i] = false;
            } else {
                self.bits[i] = true;
                return;
            }
        }
        panic!()
    }
}

fn init_vec(size: usize) -> Vec<bool> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(false);
    }
    vec
}
