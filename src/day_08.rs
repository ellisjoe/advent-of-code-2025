use crate::{EmptyResult, Result};
use std::collections::HashSet;
use std::fs;

#[test]
fn day_08_part1() -> EmptyResult {
    let (pairs, mut circuits) = parse()?;

    for (a, b, _) in pairs.iter().take(1000) {
        circuits = circuits.connect(a, b);
    }

    println!("{}", circuits.result());

    Ok(())
}

#[test]
fn day_08_part2() -> EmptyResult {
    let (pairs, mut circuits) = parse()?;

    for (a, b, _) in pairs.iter() {
        circuits = circuits.connect(a, b);
        if circuits.count() == 1 {
            println!("{}", a.x * b.x);
            return Ok(())
        }
    }

    Ok(())
}

fn parse() -> Result<(Vec<(Point, Point, i64)>, Circuits)> {
    let points: Vec<Point> = fs::read_to_string("input/day-08.txt")?
        .lines()
        .map(Point::parse)
        .collect();

    let mut pairs = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            pairs.push((
                points[i].clone(),
                points[j].clone(),
                points[i].distance(&points[j]),
            ));
        }
    }

    pairs.sort_by_key(|(_, _, d)| *d);

    let circuits = Circuits::new(points.into_iter().map(Circuit::new).collect());
    Ok((pairs, circuits))
}

struct Circuits {
    circuits: Vec<Circuit>,
}

impl Circuits {
    fn new(circuits: Vec<Circuit>) -> Self {
        Self { circuits }
    }

    fn connect(self, box_1: &Point, box_2: &Point) -> Circuits {
        let mut connected = Circuit::new_empty();
        let mut all = Vec::new();
        for circuit in self.circuits {
            if circuit.contains(box_1) || circuit.contains(box_2) {
                connected.connect(circuit);
            } else {
                all.push(circuit);
            }
        }
        all.push(connected);

        Self { circuits: all }
    }

    fn count(&self) -> usize {
        self.circuits.len()
    }

    fn result(&self) -> usize {
        let mut clone = self.circuits.clone();
        clone.sort_by_key(|c| c.count());
        clone.reverse();
        clone[0].count() * clone[1].count() * clone[2].count()
    }
}

#[derive(Clone)]
struct Circuit {
    junctions: HashSet<Point>,
}

impl Circuit {
    fn new_empty() -> Self {
        Self {
            junctions: HashSet::new(),
        }
    }

    fn new(junction: Point) -> Self {
        let mut junctions = HashSet::new();
        junctions.insert(junction);
        Self { junctions }
    }

    fn contains(&self, point: &Point) -> bool {
        self.junctions.contains(point)
    }

    fn connect(&mut self, other: Circuit) {
        self.junctions.extend(other.junctions);
    }

    fn count(&self) -> usize {
        self.junctions.len()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn parse(line: &str) -> Self {
        let input = line.split(",").collect::<Vec<&str>>();
        Self {
            x: input[0].parse().unwrap(),
            y: input[1].parse().unwrap(),
            z: input[2].parse().unwrap(),
        }
    }

    fn distance(&self, other: &Point) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}
