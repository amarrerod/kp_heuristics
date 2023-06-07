mod heuristic;
mod knapsack;

use knapsack::Knapsack;

fn main() {
    let n: u32 = 10;
    let mut knapsack = Knapsack::new(n, Some(100), None);
    let ok = knapsack.create_random_items(1, 100);

    println!("Knapsack: {:?}", knapsack);

    let result = heuristic::default(&knapsack, true);
    if let Ok(solution) = result {
        println!("{:?}", solution);
        assert!(solution.weight <= knapsack.capacity);
    }
}
//
