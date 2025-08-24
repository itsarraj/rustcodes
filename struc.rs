use std::sync::{Arc, Mutex};
use std::thread;

fn is_perfect(n: u64) -> bool {
    let mut sum = 0;
    for i in 1..=n / 2 {
        if n % i == 0 {
            sum += i;
        }
    }
    sum == n
}

fn main() {
    let max: u64 = 1_000_000_000_000; // or go higher cautiously
    let threads = 8; // number of threads
    let chunk_size = max / threads;
    let results = Arc::new(Mutex::new(vec![]));

    let mut handles = vec![];

    for i in 0..threads {
        let start = i * chunk_size + 2;
        let end = if i == threads - 1 {
            max
        } else {
            (i + 1) * chunk_size
        };

        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            for num in start..end {
                if is_perfect(num) {
                    println!("Perfect number found: {}", num);
                    results_clone.lock().unwrap().push(num);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Done.");
    let found = results.lock().unwrap();
    println!("Perfect numbers found: {:?}", *found);
}
