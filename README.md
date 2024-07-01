# spinlock-rs

Building an intuition for the cost of locks by benchmarking spin locks vs mutex locks. 

## Benchmarking 

Benchmarking is done with [Criterion](https://github.com/bheisler/criterion.rs). 

```console 
cargo bench
```

## Credits 

The implementation is derived from the spinlock implementation written by Mara Bos in Rust Atomics and Locks. 
