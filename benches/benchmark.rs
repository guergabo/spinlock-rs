extern crate criterion; 

use criterion::{criterion_group, criterion_main, Criterion};

extern crate spinlock_rs; 
use spinlock_rs::SpinLock; 

// SpinLock benchmarks 

fn spin_lock_unlock_with_no_contention_benchmark(c: &mut Criterion) {
    let spin = SpinLock::new(0); 
    c.bench_function(
        "spin_lock_unlock_with_no_contention",
        |b| b.iter(|| {
            let mut val = spin.lock(); 
            *val += 1; 
        }),
    );
}

fn spin_lock_unlock_with_contention_benchmark(c: &mut Criterion) {
    let spin = SpinLock::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..1000 {
                let mut val = spin.lock(); 
                *val += 1; 
            }
        }); 
        c.bench_function(
            "spin_lock_unlock_with_contention", 
            |b| b.iter(|| {
                let mut val = spin.lock(); 
                *val += 1; 
            }), 
        ); 
    }); 
}

// Mutex benchmarks 

use std::sync::Mutex; 

fn mutex_lock_unlock_no_contention_benchmark(c: &mut Criterion) {
    let mutex = Mutex::new(0); 
    c.bench_function(
        "mutex_lock_unlock_no_contention", 
        |b| b.iter(|| {
            let mut val = mutex.lock().unwrap(); 
            *val += 1; 
        }),
    );
}

fn mutex_lock_unlock_with_contention_benchmark(c: &mut Criterion) {
    let mutex = Mutex::new(0); 
    std::thread::scope(|s| {
        s.spawn(|| {
           for _ in 0..1000 {
            let mut val = mutex.lock().unwrap(); 
            *val += 1; 
           } 
        }); 
        c.bench_function(
            "mutex_lock_unlock_with_contention", 
            |b| b.iter(|| {
                let mut val = mutex.lock().unwrap(); 
                *val += 1; 
            }),
        );
    }); 
}

criterion_group!(
    benches, 
    spin_lock_unlock_with_no_contention_benchmark, 
    mutex_lock_unlock_no_contention_benchmark, 
    spin_lock_unlock_with_contention_benchmark, 
    mutex_lock_unlock_with_contention_benchmark, 
); 
criterion_main!(benches); 
