extern crate in_memory_cache;

use std::ops::Div;
use std::time::Instant;

const KEY: &str = "/hi/zdravo/svete/";

fn bench_with_size_n_items(size: usize, items: usize, read: usize) {
	let source = vec![1; size];

	let mut cache = in_memory_cache::Cache::with_size(size * items);

	println!("# ===");
	println!("Benchmark");
	println!("Entry size {} bytes", size);
	println!("Number of items {}", items);
	println!("Read element index {}", read);
	println!("# ===");

	let start = Instant::now();

	for i in 0..items {
		if let Err(e) = cache.add(format!("{}-{}", KEY, i), source.clone()) {
			println!("{:?}", e);
		}
	}
	println!("Add: {:?}", Instant::now().duration_since(start).div(items as u32));

	let start = Instant::now();

	for _ in 0..items {
		cache.get(format!("{}-{}", KEY, read));
	}

	println!("Read: {:?}", Instant::now().duration_since(start).div(items as u32));
	println!("# ===");
}

fn main() {
	let sizes = vec![
		1024,
		4 * 1024,
		16 * 1024,
		256 * 1024,
		1024 * 1024,
		4 * 1024 * 1024,
		16 * 1024 * 1024,
	];

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 1, 0);
		println!();
	});

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 4, 1);
		println!();
	});

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 4, 3);
		println!();
	});

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 64, 31);
		println!();
	});

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 64, 63);
		println!();
	});
}
