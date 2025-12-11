use std::collections::HashMap;

use crate::day11::models::PathCount;

pub fn solve_part_one(graph: &HashMap<String, Vec<String>>) -> i64 {
    let mut cache = HashMap::new();
    count_paths_part_one("you", graph, &mut cache)
}

fn count_paths_part_one(node: &str, graph: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, i64>) -> i64 {
    if node == "out" {
        return 1;
    }

    if let Some(value) = cache.get(node) {
        return *value;
    }

    let mut result = 0;
    for neighbor in &graph[node] {
        result += count_paths_part_one(neighbor, graph, cache);
    }

    cache.insert(node.to_string(), result);

    result
}

pub fn solve_part_two(graph: &HashMap<String, Vec<String>>) -> i64 {
    let mut cache = HashMap::new();
    count_paths_part_two("svr", graph, &mut cache).2
}

fn count_paths_part_two(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, PathCount>,
) -> PathCount {
    if node == "out" {
        return PathCount(1, 0, 0);
    }

    if let Some(value) = cache.get(node) {
        return *value;
    }

    let mut result = PathCount::new();
    for neighbor in &graph[node] {
        result += count_paths_part_two(neighbor, graph, cache);
    }

    // Shift the result if we are one of the target machine
    if node == "dac" || node == "fft" {
        result.shift();
    }

    cache.insert(node.to_string(), result);

    result
}
