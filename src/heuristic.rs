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
/// 1. An instance of the Knapsack
/// 2. A boolean flag to log the execution of the algorithm
///
/// Returns a result to a Solution struct
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

/// Runs the MaP heuristic for Knapsack Problem using the given data
///
/// The MaP heuristic for Knapsack Problem is a heuristic
/// which takes the maximum profit available item in the set to include inside the Knapsack.
///
/// Necessary parameters:
///
/// 1. An instance of the Knapsack
/// 2. A boolean flag to log the execution of the algorithm
///
/// Returns a result to a Solution struct
///
pub fn map(knapsack: &Knapsack, verbose: bool) -> Result<Solution, ()> {
    let mut in_kp = 0;
    let mut total_p = 0;
    let mut i: u32 = 0;
    let mut solution: Vec<u32> = vec![0; knapsack.n as usize];

    let mut items = knapsack
        .profits
        .iter()
        .zip(knapsack.weights.clone())
        .enumerate()
        .map(|(i, (p, w))| (*p, w, i))
        .collect::<Vec<(u32, u32, usize)>>();

    items.sort_by(|a, b| a.0.cmp(&b.0));
    items.reverse();

    loop {
        if i >= knapsack.n {
            break;
        }
        if items[i as usize].1 + in_kp <= knapsack.capacity {
            in_kp += items[i as usize].1;
            total_p += items[i as usize].0;
            // We get the index of the item in the original knapsack
            solution[items[i as usize].2] = 1;
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

/// Runs the MiW heuristic for Knapsack Problem using the given data
///
/// The MiW heuristic for Knapsack Problem is a heuristic
/// which takes the minimum weight available item in the set to include inside the Knapsack.
///
/// Necessary parameters:
///
/// 1. An instance of the Knapsack
/// 2. A boolean flag to log the execution of the algorithm
///
/// Returns a result to a Solution struct
///
pub fn miw(knapsack: &Knapsack, verbose: bool) -> Result<Solution, ()> {
    let mut in_kp = 0;
    let mut total_p = 0;
    let mut i: u32 = 0;
    let mut solution: Vec<u32> = vec![0; knapsack.n as usize];

    let mut items = knapsack
        .profits
        .iter()
        .zip(knapsack.weights.clone())
        .enumerate()
        .map(|(i, (p, w))| (*p, w, i))
        .collect::<Vec<(u32, u32, usize)>>();

    items.sort_by(|a, b| a.1.cmp(&b.1));

    loop {
        if items[i as usize].1 + in_kp <= knapsack.capacity {
            in_kp += items[i as usize].1;
            total_p += items[i as usize].0;
            // We get the index of the item in the original knapsack
            solution[items[i as usize].2] = 1;
        } else {
            break;
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

/// Runs the MPW heuristic for Knapsack Problem using the given data
///
/// The MPW heuristic for Knapsack Problem is a heuristic
/// which takes the maximum per weight available item in the set to include inside the Knapsack.
///
/// Necessary parameters:
///
/// 1. An instance of the Knapsack
/// 2. A boolean flag to log the execution of the algorithm
///
/// Returns a result to a Solution struct
///
pub fn mpw(knapsack: &Knapsack, verbose: bool) -> Result<Solution, ()> {
    let mut in_kp = 0;
    let mut total_p = 0;
    let mut i: u32 = 0;
    let mut solution: Vec<u32> = vec![0; knapsack.n as usize];

    let mut items: Vec<(u32, u32, usize, f64)> = knapsack
        .profits
        .iter()
        .zip(knapsack.weights.clone())
        .enumerate()
        .map(|(i, (p, w))| (*p, w, i, (*p as f64 / w as f64)))
        .collect::<Vec<(u32, u32, usize, f64)>>();

    items.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    items.reverse();

    loop {
        if i >= knapsack.n {
            break;
        }
        if items[i as usize].1 + in_kp <= knapsack.capacity {
            in_kp += items[i as usize].1;
            total_p += items[i as usize].0;
            // We get the index of the item in the original knapsack
            solution[items[i as usize].2] = 1;
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
