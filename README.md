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
6.25 real         6.11 user         0.05 sys  
bench: 514,542,138 ns/iter (+/- 33,268,484)  

### v1.0.1 - Minor tweaks in the single threaded code
Avoid writing 0 to pixel buffer when not in set, as it's already initialized to zero
6.19 real         6.05 user         0.05 sys  
bench: 513,143,436 ns/iter (+/- 37,743,921)  
bench: 510,725,605 ns/iter (+/- 56,052,267)  

### v1.0.2 - Another minor tweak in the single threaded code
Simplify the index arrithmetic in addressing the pixel buffer, avoiding 
multiplies and adds and just incrementing an offset into the pixel buffer.

6.14 real         5.98 user         0.06 sys
bench: 502,205,421 ns/iter (+/- 92,783,074)
bench: 511,686,945 ns/iter (+/- 62,674,916)
bench: 504,082,705 ns/iter (+/- 21,991,316)