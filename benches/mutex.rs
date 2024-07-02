extern crate criterion; 
use criterion::{criterion_group, criterion_main, Criterion};

use std::thread::sleep; 
use std::time::Duration; 

use std::sync::Mutex; 

// Mutex benchmarks 

fn mutex_lock_unlock_no_contention_benchmark(c: &mut Criterion) {
    let mutex = Mutex::new(0); 
    c.bench_function(
        "mutex_lock_unlock_no_contention", 
        |b| b.iter(|| {
            // let mut val = mutex.lock().unwrap(); 
            // *val += 1; 
            if let Ok(mut val) = mutex.lock() {
                *val += 1; 
            }
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

fn mutex_lock_unlock_with_contention_and_long_critical_section_benchmark(c: &mut Criterion) {
    let mutex = Mutex::new(0); 
    std::thread::scope(|s| {
        s.spawn(|| {
           for _ in 0..1000 {
            let mut val = mutex.lock().unwrap(); 
            *val += 1; 
            sleep(Duration::from_micros(100)); // Simulate long critical section 
           } 
        }); 
        c.bench_function(
            "mutex_lock_unlock_with_contention_and_long_critical_section", 
            |b| b.iter(|| {
                let mut val = mutex.lock().unwrap(); 
                *val += 1; 
                sleep(Duration::from_micros(100)); // Simulate long critical section 
            }),
        );
    }); 
}

criterion_group!(
    name = mutex;  
    config = Criterion::default(); 
    targets = mutex_lock_unlock_no_contention_benchmark, 
        mutex_lock_unlock_with_contention_benchmark, 
        mutex_lock_unlock_with_contention_and_long_critical_section_benchmark  
); 

criterion_main!(mutex); 
