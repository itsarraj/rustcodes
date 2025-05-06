use std::thread;

// Check if a number is prime (simple method)
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for i in (3..=((n as f64).sqrt() as u64)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Generate perfect number using Euclid's formula if 2^p - 1 is prime
fn find_perfect_number(p: u64) -> Option<u128> {
    let mersenne = (1u128 << p) - 1;
    if is_prime(mersenne as u64) {
        let perfect = (1u128 << (p - 1)) * mersenne;
        Some(perfect)
    } else {
        None
    }
}

fn main() {
    let upper_p: u64 = 60; // You can increase this to 60–70 depending on performance
    let mut handles = vec![];

    for p in 2..=upper_p {
        let handle = thread::spawn(move || {
            if let Some(perfect) = find_perfect_number(p) {
                println!("Perfect number for p = {:2}: {}", p, perfect);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    println!("done");
}
