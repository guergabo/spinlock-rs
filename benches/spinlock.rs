extern crate criterion; 
use criterion::{criterion_group, criterion_main, Criterion};

extern crate pprof; 
use pprof::criterion::{Output, PProfProfiler};

extern crate spinlock_rs; 
use spinlock_rs::SpinLock; 

use std::thread::sleep; 
use std::time::Duration; 

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

fn spin_lock_unlock_with_contention_and_long_critical_section_benchmark(c: &mut Criterion) {
    let spin = SpinLock::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..1000 {
                let mut val = spin.lock(); 
                *val += 1; 
                sleep(Duration::from_micros(100)); // Simulate long critical section 
            }
        }); 
        c.bench_function(
            "spin_lock_unlock_with_contention_and_long_critical_section", 
            |b| b.iter(|| {
                let mut val = spin.lock(); 
                *val += 1; 
                sleep(Duration::from_micros(100)); // Simulate long critical section 
            }), 
        ); 
    }); 
}


criterion_group!(
    name = spinlock; 
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = spin_lock_unlock_with_no_contention_benchmark, 
        spin_lock_unlock_with_contention_benchmark, 
        spin_lock_unlock_with_contention_and_long_critical_section_benchmark, 
); 

criterion_main!(spinlock); 
