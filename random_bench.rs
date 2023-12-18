use std::time::Instant;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use thousands::Separable;

fn main() {
    let rewards = vec![1, 2, 3, 4];
    let length = rewards.len();
    let total_percent = rewards.iter().sum::<i32>() as f64;
    let mut count_by_id = vec![0; length];
    let run_count = 100000000;

    let start = Instant::now();

    // slower but more secure let mut rng = thread_rng();
    // for more information, see https://rust-random.github.io/book/guide-rngs.html
    let mut rng = SmallRng::from_entropy();
    for _ in 0..run_count {
        let mut random: f64 = rng.gen::<f64>() * total_percent;
        for i in 0..length {
            random -= rewards[i] as f64;
            if random <= 0f64 {
                count_by_id[i] += 1;
                break;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Total run: {}, elapsed: {:.2}s. {:.2} ns/run, {} runs/s", run_count.separate_with_commas(), elapsed.as_secs_f64(), elapsed.as_nanos() as f64 / run_count as f64, (run_count / elapsed.as_millis() * 1000).separate_with_commas());

    for (i, count) in count_by_id.iter().enumerate() {
        println!("id: {}, count: {} ({:.6}%)", i, count, *count as f64 / run_count as f64 * 100f64);
    }
}
