use rand::{distributions::Uniform, Rng};
mod heuristic;

fn main() {
    let percentage: f32 = 0.6;
    let n = 10000;
    let range = Uniform::from(1..100);
    let profits: Vec<i32> = rand::thread_rng().sample_iter(&range).take(n).collect();
    let weights: Vec<i32> = rand::thread_rng().sample_iter(&range).take(n).collect();
    let q = weights.iter().sum::<i32>();
    let Q: f32 = percentage * q as f32;
    let result = heuristic::default(&n, &profits, &weights, &(Q as i32), true);
    println!("{:?}", result);
    assert!(result.1 <= Q as i32);
}
