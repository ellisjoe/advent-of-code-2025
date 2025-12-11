use crate::EmptyResult;
use geo::{Contains, Coord, LineString};
use geo::Rect;
use std::fs;

#[test]
fn day_09_part1() -> EmptyResult {
    let points: Vec<Point> = fs::read_to_string("input/day-09.txt")?
        .lines()
        .map(Point::parse)
        .collect();

    let mut max = 0f64;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            max = max.max(points[i].area(&points[j]));
        }
    }

    println!("{}", max);
    Ok(())
}

#[test]
fn day_09_part2() -> EmptyResult {
    let mut points: Vec<_> = fs::read_to_string("input/day-09.txt")?
        .lines()
        .map(Point::parse)
        .map(|p| Coord::from((p.x, p.y)))
        .collect();
    points.push(points[0].clone());

    let polygon = geo::Polygon::new(LineString::from(points.clone()), vec![]);

    let mut max = 0f64;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let rect = Rect::new(points[i].clone(), points[j].clone());
            let area = (rect.width() + 1f64) * (rect.height() + 1f64);
            if area > max && !points.iter().any(|p| rect.contains(p)) && polygon.contains(&rect) {
                max = area;
            }
        }
    }

    println!("{}", max);
    Ok(())
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn parse(input: &str) -> Self {
        let parts: Vec<i64> = input.split(",").map(|x| x.parse().unwrap()).collect();
        Self {
            x: parts[0] as f64,
            y: parts[1] as f64,
        }
    }

    fn area(&self, point: &Point) -> f64 {
        let min_x = self.x.min(point.x);
        let max_x = self.x.max(point.x);
        let min_y = self.y.min(point.y);
        let max_y = self.y.max(point.y);

        (max_x - min_x + 1f64) * (max_y - min_y + 1f64)
    }
}
