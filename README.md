# spinlock-rs

Building an intuition for the cost of locks by benchmarking spin locks vs mutex locks. 

## Benchmarking

Benchmarking is done with [Criterion](https://github.com/bheisler/criterion.rs).  

```console 
cargo clean && cargo bench --color always
```

## CPU Profiling

CPU profiling is done with [pprof-rs](https://github.com/tikv/pprof-rs).

```console 
cargo clean && sudo cargo bench --color always --bench benchmark -- --profile-time=5
```

## Credits 

The implementation is derived from the spinlock implementation written by Mara Bos in Rust Atomics and Locks. 
