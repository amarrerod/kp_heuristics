mod heuristic;
mod knapsack;

use knapsack::Knapsack;

fn main() {
    let n: u32 = 100;
    let mut knapsack = Knapsack::new(n, Some(500), None);
    let _ = knapsack.create_random_items(1, 100);

    println!("Knapsack: {:?}", knapsack);

    let result_default = heuristic::default(&knapsack, false);
    if let Ok(solution) = result_default {
        println!("{:?}", solution);
        assert!(solution.weight <= knapsack.capacity);
    }

    let result_map: Result<knapsack::Solution, ()> = heuristic::map(&knapsack, false);
    if let Ok(solution) = result_map {
        println!("{:?}", solution);
        assert!(solution.weight <= knapsack.capacity);
    }

    let result_miw: Result<knapsack::Solution, ()> = heuristic::miw(&knapsack, false);
    if let Ok(solution) = result_miw {
        println!("{:?}", solution);
        assert!(solution.weight <= knapsack.capacity);
    }

    let result_mpw: Result<knapsack::Solution, ()> = heuristic::mpw(&knapsack, false);
    if let Ok(solution) = result_mpw {
        println!("{:?}", solution);
        assert!(solution.weight <= knapsack.capacity);
    }
}
//
