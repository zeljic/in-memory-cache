extern crate in_memory_cache;

use in_memory_cache::Cache;

use std::{
	ops::{Div, Range},
	time::Instant,
};

static KEY_PREFIX: &str = "few:levels:deep:key-";

fn populate(cache: &mut Cache, items: usize, data_size: usize, prefix: &str) {
	for idx in 0..items {
		if cache.add(format!("{}-{}", prefix, idx), vec![1; data_size]).is_ok() {}
	}
}

enum TYPE {
	Capacity,
	Size,
}

fn perf_add_read(t: TYPE, value: usize, data_size: usize, range: Range<usize>) {
	let mut cache = match t {
		TYPE::Capacity => Cache::with_capacity(value),
		TYPE::Size => Cache::with_size(value * data_size),
	};

	populate(&mut cache, value, data_size, KEY_PREFIX);

	let format_range = format!("{:?}", &range);
	let range_len = range.len() as u32;

	let rev = range.to_owned().rev();
	let rev_len = rev.len() as u32;

	let start = Instant::now();

	for idx in range {
		if cache.get(format!("{}{}", KEY_PREFIX, idx)).is_some() {}
	}

	let duration_until = Instant::now().duration_since(start);

	println!(
		"{:10}   {:10}   {:10}   {:10}   {:32}",
		value,
		data_size,
		format!("{:?}", duration_until),
		format!("{:?}", duration_until.div(range_len)),
		format_range
	);

	//

	let mut cache = match t {
		TYPE::Capacity => Cache::with_capacity(value),
		TYPE::Size => Cache::with_size(value * data_size),
	};

	populate(&mut cache, value, data_size, KEY_PREFIX);

	let format_range = format!("{:?}", &rev);

	let start = Instant::now();

	for idx in rev {
		if cache.get(format!("{}{}", KEY_PREFIX, idx)).is_some() {}
	}

	let duration_until = Instant::now().duration_since(start);

	println!(
		"{:10}   {:10}   {:10}   {:10}   {:32}",
		value,
		data_size,
		format!("{:?}", duration_until),
		format!("{:?}", duration_until.div(rev_len)),
		format_range
	);
}

fn main() {
	println!("# TYPE: CAPACITY");
	println!(
		"{:10}   {:10}   {:10}   {:10}   {:32}",
		"ITEMS", "SIZE", "SUM", "AVERAGE", "RANGE"
	);

	perf_add_read(TYPE::Capacity, 64, 256 * 1024, 0..63);
	perf_add_read(TYPE::Capacity, 64, 256 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Capacity, 64, 1024 * 1024, 0..63);
	perf_add_read(TYPE::Capacity, 64, 1024 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Capacity, 64, 2 * 1024 * 1024, 0..63);
	perf_add_read(TYPE::Capacity, 64, 2 * 1024 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Capacity, 64, 4 * 1024 * 1024, 0..63);
	perf_add_read(TYPE::Capacity, 64, 4 * 1024 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Capacity, 64, 8 * 1024 * 1024, 0..63);
	perf_add_read(TYPE::Capacity, 64, 8 * 1024 * 1024, 26..36);

	///////////////////////////////////////////////////////////

	println!();
	println!();

	println!("# TYPE: SIZE");
	println!(
		"{:10}   {:10}   {:10}   {:10}   {:32}",
		"ITEMS", "SIZE", "SUM", "AVERAGE", "RANGE"
	);

	perf_add_read(TYPE::Size, 64, 256 * 1024, 0..63);
	perf_add_read(TYPE::Size, 64, 256 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Size, 64, 1024 * 1024, 0..63);
	perf_add_read(TYPE::Size, 64, 1024 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Size, 64, 2 * 1024 * 1024, 0..63);
	perf_add_read(TYPE::Size, 64, 2 * 1024 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Size, 64, 4 * 1024 * 1024, 0..63);
	perf_add_read(TYPE::Size, 64, 4 * 1024 * 1024, 26..36);

	println!();

	perf_add_read(TYPE::Size, 64, 8 * 1024 * 1024, 0..63);
	perf_add_read(TYPE::Size, 64, 8 * 1024 * 1024, 26..36);
}
