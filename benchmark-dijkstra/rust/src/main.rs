use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i32)>>) {
    let n = graph.len();
    let mut dist = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for &(next, weight) in &graph[position] {
            let next_cost = cost + weight;

            if next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push(State { cost: next_cost, position: next });
            }
        }
    }
}

fn main() {
    let n = 100_000_000;
    let mut graph = vec![Vec::new(); n];

    for i in 0..n - 1 {
        graph[i].push((i + 1, 1));
    }

    let start = Instant::now();

    dijkstra(0, &graph);

    let duration = start.elapsed();
    println!("Rust Dijkstra: {:?}", duration);
}