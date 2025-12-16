use crate::EmptyResult;
use std::collections::HashMap;
use std::fs;

#[test]
fn day_11_part1() -> EmptyResult {
    let input = fs::read_to_string("input/day-11.txt");
    let input: HashMap<String, Vec<String>> = input?.lines().map(parse).collect();

    let conns = Connections { connections: input };

    let out = conns.find_paths("you", "out");

    println!("{out:?}");
    Ok(())
}

#[test]
fn day_11_part2() -> EmptyResult {
    let input = fs::read_to_string("input/day-11.txt");
    let input: HashMap<String, Vec<String>> = input?.lines().map(parse).collect();

    let conns = Connections { connections: input };

    let out = conns.find_paths("svr", "dac")
        * conns.find_paths("dac", "fft")
        * conns.find_paths("fft", "out");

    let out_2 = conns.find_paths("svr", "fft")
        * conns.find_paths("fft", "dac")
        * conns.find_paths("dac", "out");

    println!("{}", out + out_2);
    Ok(())
}

struct Connections {
    connections: HashMap<String, Vec<String>>,
}

impl Connections {
    fn find_paths(&self, start: &str, end: &str) -> i64 {
        self.find_paths_cached(start, end, &mut HashMap::new())
    }

    fn find_paths_cached(&self, start: &str, end: &str, cache: &mut HashMap<String, i64>) -> i64 {
        if start == end {
            return 1;
        }

        if let Some(count) = cache.get(start) {
            *count
        } else {

            let Some(out) = self.connections.get(start) else {
                return 0;
            };

            let ret = out
                .iter()
                .map(|o| self.find_paths_cached(o, end, cache))
                .sum();
            cache.insert(start.to_string(), ret);
            ret
        }
    }
}

fn parse(input: &str) -> (String, Vec<String>) {
    let (left, right) = input.split_once(": ").unwrap();
    let outputs = right.split(" ").map(String::from).collect::<Vec<_>>();
    (left.to_string(), outputs)
}
