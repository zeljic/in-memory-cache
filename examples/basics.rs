extern crate in_memory_cache;

use std::time::Instant;

fn bench_with_size_n_items(size: usize, items: usize, read: usize) -> () {
	let source = vec![1; size];

	let mut cache = in_memory_cache::Cache::with_size(size * items);

	let key = "z";

	println!("# ===");
	println!("Benchmark");
	println!("Entry size {} bytes", size);
	println!("Number of items {}", items);
	println!("Read element index {}", read);

	let start = Instant::now();

	for i in 0..items {
		if let Err(e) = cache.add(format!("{}-{}", key, i), source.clone()) {
			println!("{:?}", e);
		}
	}

	println!("Measure add: {:?}", Instant::now().duration_since(start));

	let start = Instant::now();

	cache.get(format!("{}-{}", key, read));

	println!("Measure read: {:?}", Instant::now().duration_since(start));
	println!("# ===");
}

fn main() {
	let sizes = vec![
		1 * 1024,
		4 * 1024,
		16 * 1024,
		256 * 1024,
		1 * 1024 * 1024,
		4 * 1024 * 1024,
		16 * 1024 * 1024,
	];

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 1, 0);
		println!();
	});

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 4, 0);
		println!();
	});

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 4, 2);
		println!();
	});

	sizes.iter().for_each(|source| {
		bench_with_size_n_items(*source, 64, 16);
		println!();
	});
}
