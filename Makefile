measure:
	cargo +nightly build --release
	time target/release/mandlebrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
	cargo +nightly bench