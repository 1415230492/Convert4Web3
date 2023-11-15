use rand::{thread_rng, Rng};

pub fn random_number(max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..=(max as u128 - 1)) as usize
}