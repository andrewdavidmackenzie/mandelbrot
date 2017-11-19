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
- Processor 2.4GHz Intel Core i5  (that should be 4 cores, not HyperThreading, I understand)
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

### v2.0.0 - Multithreaded version using crossbeam crate (as per book)
In this version I have updated the core to match latest version of the book, 
and implemented multi-threading using crossbeam.

Let's experiment with the number of threads to see how it varies (on this MBP):
 1: 6.02 real         5.91 user         0.05 sys  
 2: 5.42 real         5.74 user         0.07 sys  
 3: 4.23 real         5.74 user         0.04 sys  
 4: 3.22 real         5.68 user         0.03 sys  
 5: 2.64 real         5.69 user         0.03 sys  
 6: 2.54 real         5.70 user         0.05 sys
 7: 2.12 real         5.73 user         0.04 sys  
 8: 1.92 real         5.76 user         0.03 sys  
 9: 1.86 real         5.79 user         0.03 sys  
10: 1.75 real         5.80 user         0.03 sys  <<<<<
11: 1.76 real         5.85 user         0.03 sys  
12: 1.85 real         5.83 user         0.05 sys  
 
Using multi-threading the real (wall-clock) time has been reduced from 6.2s to 1.75s, 
or 3.5 times speedup

It's interesting that on a 4-core (non-HyperThreaded) machine that the optimal number of cores
is 10... But there's a bit of fixed (single-threaded) time in writing the file. 

To get to the optimal number for the rendering part, in next part I will refactor render to
take number of threads as a parameter and write benchmark tests for the different values.

Here is a set of baseline bench numbers for the different functions before in future veersions
I play with them and multi-threading in render more.

bench_escapes             ... bench:          32 ns/iter (+/- 8)  
bench_pixel_to_point      ... bench:           0 ns/iter (+/- 0)  
bench_render_1000_by_1000 ... bench: 493,086,861 ns/iter (+/- 12,176,118)  
