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
pub fn default(
    n: &usize,
    profits: &Vec<i32>,
    weights: &Vec<i32>,
    Q: &i32,
    verbose: bool,
) -> (i32, i32) {
    assert_eq!(profits.len(), *n as usize);
    assert_eq!(weights.len(), *n as usize);

    let mut done = false;
    let mut in_kp = 0;
    let mut total_p = 0;
    let mut i: usize = 0;
    let mut inside_items = 0;
    while !done {
        if i >= *n {
            done = true;
        } else if weights[i] + in_kp <= *Q {
            in_kp += weights[i];
            total_p += profits[i];
            inside_items += 1;
        }
        if verbose {
            println!(
                "I: {}, In_kp: {}, Total_p : {}, Items inside: {}",
                i, in_kp, total_p, inside_items
            );
        }
        i += 1;
    }
    (total_p, in_kp)
}
