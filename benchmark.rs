#![allow(dead_code)]
#![allow(unused)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{self, Instant};
use std::hint::black_box;
use rand::{SeedableRng, rngs::SmallRng};
use rand::{Rng, RngExt};

fn sequential_read(data: &[u64]) -> u64 {
    let mut sum: u64 = 0;
    for &n in data {
        sum += black_box(n);
    }
    sum
}

fn random_read(data: &[u64], indices: &[usize]) -> u64 {
    let mut sum: u64 = 0;
    for &i in indices {
        sum += black_box(data[i]);
    }
    sum
}

fn sequential_vs_random_access() {
    let mut total_elem: u64 = 67_108_864;
    let mut data: Vec<u64> = vec![1; total_elem as usize];
    
    let start = std::time::Instant::now();
    let sum = sequential_read(&data);
    let elapsed1 = start.elapsed().as_secs_f64();

    println!("sequential_read:\nSum: {} | elapsed: {elapsed1:?}", sum);

    let mut rng: SmallRng = rand::make_rng();

    let indices: Vec<usize> = (0..total_elem)
        .map(|_| rng.random_range(0..total_elem as usize))
        .collect();

    
    let start = std::time::Instant::now();
    let sum = random_read(&data, &indices);
    let elapsed2 = start.elapsed().as_secs_f64();

    println!("random read:\nSum: {} | elapsed: {:?}", sum, elapsed2);

    let times_longer = (elapsed2 / elapsed1);
    let percentage_slower = (times_longer - 1.) * 100.;
    println!("Percentage slower: {:.3}%\nTimes Longer: {:.3}x", percentage_slower, times_longer);

    let time_per_elem_seq = (elapsed1 / total_elem as f64) * 1_000_000_000.;
    let time_per_elem_rand = (elapsed2 / total_elem as f64) * 1_000_000_000.;
    println!("Time per element(Sequential read) = {time_per_elem_seq:.3}ns");
    println!("Time per element(Random read) = {time_per_elem_rand:.3}ns");
}

struct Counters {
    a: AtomicU64,
    b: AtomicU64,
}

#[repr(C, align(64))]
struct PaddedCounter {
    value: AtomicU64,
    _pad: [u8; 56],
}

fn false_sharing_measurement() {
    let counters = Counters { 
        a: AtomicU64::new(0),
        b: AtomicU64::new(0) 
    };

    let start = Instant::now();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..100_000_000 {
                counters.a.fetch_add(1, Ordering::Relaxed);
            }
        });

        scope.spawn(|| {
            for _ in 0..100_000_000 {
                counters.b.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
    
    let elapsed = start.elapsed().as_secs_f32();
    println!("False Sharing:\nElapsed: {elapsed:?}\nFinal A: {} | Final B: {}", counters.a.load(Ordering::Relaxed), counters.b.load(Ordering::Relaxed));

    let counter1 = PaddedCounter {
        value: AtomicU64::new(0),
        _pad: [0; 56],
    };
    
    let counter2 = PaddedCounter {
        value: AtomicU64::new(0),
        _pad: [0; 56],
    };

    let start = Instant::now();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..100_000_000 {
            counter1.value.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
    
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..100_000_000 {
            counter2.value.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
    let elapsed1 = start.elapsed().as_secs_f32();
    let times_longer = elapsed / elapsed1;
    let percentage_slower = (times_longer - 1.) * 100.;
    println!("Independent:\nElapsed: {elapsed1:.3}\nFinal C1: {} | Final C2: {}\n", 
        counter1.value.load(Ordering::Relaxed), counter2.value.load(Ordering::Relaxed));
    println!("Times longer: {times_longer:?}x\nPercentage slower: {percentage_slower:?}%");
    let time_per_elem1_fs = (elapsed / 200_000_000.) * 1_000_000_000.;
    let time_per_elem1 = (elapsed1 / 200_000_000.) * 1_000_000_000.;
    println!("False sharing: {time_per_elem1_fs:.3}ns");
    println!("Independent: {time_per_elem1:.3}ns");
}
fn main() {
    sequential_vs_random_access();
    false_sharing_measurement();
}