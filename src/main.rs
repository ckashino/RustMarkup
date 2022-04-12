use std::time::{Instant, Duration};

static SYMBOLS: [&str; 2] = ["-", "#"];

fn get_symbol(string: &str) -> i32 {
	let mut counter = -1;
	for i in SYMBOLS.into_iter() {
		counter = counter + 1;
		if string == i {
			return counter
		}
	}
	counter
}

fn main() {

	let start_time = Instant::now();
	
	for _ in [0; 50000]{
    	let test_val = get_symbol("#");
	}

	let ellapsed_time = start_time.elapsed().as_millis();
	
	println!("{}", ellapsed_time);
}