# spinlock-rs

Building an intuition for the cost of locks by benchmarking spin locks vs mutex locks. 

## Benchmarking

Benchmarking is done with [Criterion](https://github.com/bheisler/criterion.rs).  

spinlock: 

```console 
cargo clean && cargo bench --color always --bench spinlock
```

mutex: 

```console 
cargo clean && cargo bench --color always --bench mutex
```

## CPU Profiling

CPU profiling is done with [pprof-rs](https://github.com/tikv/pprof-rs). Currently, it only works for spinlock on my M1 MAc. Running CPU profiler with the mutex variant results in the following [known issue](https://github.com/tikv/pprof-rs/issues/237). 

```console 
cargo clean && cargo bench --color always --bench spinlock -- --profile-time=5
```

## Credits 

The implementation is derived from the spinlock implementation written by Mara Bos in Rust Atomics and Locks. 
