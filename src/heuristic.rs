use crate::knapsack::Knapsack;
use crate::knapsack::Solution;
/// Runs the Default heuristic for Knapsack Problem using the given data
///
/// The Default heuristic for Knapsack Problem is a Greedy heuristic
/// which takes the first available item in the set to include inside the Knapsack.
/// It must go through all items to properly get a solution.
///
/// Necessary parameters:
///
/// 1. The number of items in the instance (n)
/// 2. A vector of i32 numbers representing the profits of each item
/// 3. A vector of i32 numbers representing the weights of each item
/// 4. The total capacity of the Knapsack
/// 5. A boolean flag to log the execution of the algorithm
///
/// Returns a tuple with the total profit and the total weights of the solution.
///
pub fn default(knapsack: &Knapsack, verbose: bool) -> Result<Solution, ()> {
    let mut in_kp = 0;
    let mut total_p = 0;
    let mut i: u32 = 0;
    let mut solution: Vec<u32> = vec![0; knapsack.n as usize];
    loop {
        if i >= knapsack.n {
            break;
        }
        if knapsack.weights[i as usize] + in_kp <= knapsack.capacity {
            in_kp += knapsack.weights[i as usize];
            total_p += knapsack.profits[i as usize];
            solution[i as usize] = 1;
        }

        if verbose {
            println!(
                "I: {}, In_kp: {}, Total_p : {}, Solution: {:?}",
                i, in_kp, total_p, solution
            );
        }
        i += 1;
    }

    Ok(Solution::new(total_p, in_kp, &solution))
}
