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

NOTE: I think there is a bug in the multi-threaded implementation in the book, as if you run with more than one
thread the output file is different! I thought this was due to number of rows per band and tried to fix it, but 
haven't been able to so far...

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

### v2.1.0 - Some tweaks to the multithreaded crossbeam version
Implemented a set of bench tests that measure the timing for rendering with a different number
of threads. It takes quite a while to run them all, but makes it easy to get a check
of how the different number of threads perform.

Before any tweaks to the code this were the initial results:

test tests::bench_escapes                          ... bench:          32 ns/iter (+/- 13)  
test tests::bench_pixel_to_point                   ... bench:           0 ns/iter (+/- 0)  
test tests::bench_render_threaded_01t_1000_by_1000 ... bench: 488,402,034 ns/iter (+/- 33,423,302)  
test tests::bench_render_threaded_02t_1000_by_1000 ... bench: 436,534,007 ns/iter (+/- 9,142,246)  
test tests::bench_render_threaded_04t_1000_by_1000 ... bench: 268,980,366 ns/iter (+/- 42,662,520)  
test tests::bench_render_threaded_08t_1000_by_1000 ... bench: 163,316,199 ns/iter (+/- 16,115,325)  
test tests::bench_render_threaded_10t_1000_by_1000 ... bench: 149,431,902 ns/iter (+/- 11,977,095)  
test tests::bench_render_threaded_12t_1000_by_1000 ... bench: 144,807,106 ns/iter (+/- 18,223,768)  


Minor adjustment to "escapes" to avoid a multiple and add on first iteration,
reduces time per iteration from 32ns to 29ns Wow... :-) 

Extracted the width/Height calculations out of pixel_to_point(), so that when used in a loop they are
done only once.
That causes time per iteration for 'escaped()' to drop further to 27ns (from 29ns, from 32ns).

Also, I implemented a bench of the core render algorithm, but on a smaller 100x100 image
so it didn't take to long for bench to run many iterations on it.

1  Thread: 5.95 real         5.79 user         0.05 sys  
10 Threads 1.77 real         5.67 user         0.03 sys  

test tests::bench_escapes                          ... bench:          31 ns/iter (+/- 11)  
test tests::bench_render_100_by_100                ... bench:   5,721,979 ns/iter (+/- 1,813,252)  
test tests::bench_render_threaded_01t_1000_by_1000 ... bench: 494,512,804 ns/iter (+/- 20,937,880)  
test tests::bench_render_threaded_02t_1000_by_1000 ... bench: 447,243,830 ns/iter (+/- 99,879,150)  
test tests::bench_render_threaded_04t_1000_by_1000 ... bench: 262,506,287 ns/iter (+/- 49,336,468)  
test tests::bench_render_threaded_08t_1000_by_1000 ... bench: 151,689,600 ns/iter (+/- 5,502,388)  
test tests::bench_render_threaded_10t_1000_by_1000 ... bench: 140,472,366 ns/iter (+/- 17,968,631)  
test tests::bench_render_threaded_12t_1000_by_1000 ... bench: 138,224,777 ns/iter (+/- 25,675,038)  

