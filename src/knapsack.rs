use rand::{distributions::Uniform, Rng};

#[derive(Debug, Default)]
pub struct Knapsack {
    pub n: u32,
    pub capacity: u32,
    pub weights: Vec<u32>,
    pub profits: Vec<u32>,
}

impl Knapsack {
    pub fn new(n: u32, q: Option<u32>, items: Option<&[(u32, u32)]>) -> Self {
        let mut capacity: u32 = 0;
        if q.is_some() {
            capacity = q.unwrap();
        }

        if let Some(items_data) = items {
            let mut weights: Vec<u32> = vec![];
            let mut profits: Vec<u32> = vec![];

            items_data.iter().for_each(|(p, w)| {
                profits.push(*p);
                weights.push(*w);
            });

            Knapsack {
                n,
                capacity,
                weights,
                profits,
            }
        } else {
            Knapsack {
                n,
                capacity,
                ..Default::default()
            }
        }
    }

    pub fn create_random_items(&mut self, lower_bound: u32, upper_bound: u32) -> Result<(), ()> {
        let range = Uniform::from(lower_bound..=upper_bound);

        self.profits = rand::thread_rng()
            .sample_iter(&range)
            .take(self.n as usize)
            .collect();
        self.weights = rand::thread_rng()
            .sample_iter(&range)
            .take(self.n as usize)
            .collect();

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Solution {
    pub profits: u32,
    pub weight: u32,
    pub chromosome: Vec<u32>,
}

impl Solution {
    pub fn new(profits: u32, weight: u32, chromosome: &[u32]) -> Self {
        Solution {
            profits,
            weight,
            chromosome: chromosome.to_vec(),
        }
    }
}
