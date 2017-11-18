# Mandelbrot

This is a simple implementation of a program to plot a part of the Mandelbrot set in 8bit greyscale, 
starting with the single threaded implementation in the "Programming Rust" book, as an exercise in learning rust.

I will then implement it using the multi-threaded example in the book, and finally
I'll convert that to use the rayon crate, in order to learn to use rayon also.

I'll not down the real & user times from running it on my MacBook Pro, and also
implement some bench tests and post the output of them also.

I'll do a release in github of each implementation.

## Building, Running, Testing
This requires rust nightly in order to run rust benchmarks (see below).

Install nightly using: "rustup install nightly"

If you don't want to make nightly the default for all your projects, then just tell cargo to use nightly in
any cargo command you use:
- cargo +nightly build
- cargo +nightly test
- cargo +nightly run --release -- mandel.png 4000x3000 -1.20,0.35 -1,0.20

I've written some tests that take a while to run. They are marked with #[test] #[ignore] in the
source, and they will not be run by default when you run 'cargo test'. 
However, you can have them run using:
- cargo +nightly test -- --ignored

## Benchmarks
Run on:
- MacBook Pro (Retina, 13-inch, Late 2013)
- Processor 2.4GHz Intel Core i5
- Memory 16 GB 1600 MHz DDR3
- macOS High Sierra

Real/User timings were done using:
- time target/release/mandlebrot mandel.png 4000x3000 -1.20,0.35 -1,0.20

Rust bench tests were run on render() function, using a smaller image size (1000x1000 pixels)
(as it is run many times by the benchmarking framework).

To run the benchmark test use one of these two: 
- cargo +nightly bench
- rustup run nightly cargo bench

### v1.0.0 - Initial Single Threaded Implementation
real    0m6.250s  
user    0m6.109s  
test tests::bench_render_1000_by_1000 ... bench: 514,542,138 ns/iter (+/- 33,268,484)  