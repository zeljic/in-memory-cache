use criterion::{criterion_group, criterion_main, Criterion};
use in_memory_cache::Cache;

pub fn populate(cache: &mut Cache, n: usize, size: usize) {
	for i in 0..n {
		if cache.add(format!("key-{}", i), vec![1; size]).is_ok() {}
	}
}

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("cache_add", |b| {
		let mut cache = Cache::with_size_mb(4);
		let mut idx = 0;

		b.iter(|| {
			if cache.add(format!("key-{}", idx), vec![1; 1024]).is_ok() {};
			idx += 1;
		});
	});

	c.bench_function("cache_get", |b| {
		let n = 64;
		let mut cache = Cache::with_size_kb(n as usize);

		let start_range = (n as f32 * 0.25).round() as u32;
		let end_range = (n as f32 * 0.75).round() as u32;

		let mut idx = start_range;

		populate(&mut cache, n as usize, 1024);

		b.iter(|| {
			let _ = cache.get(format!("key-{}", idx)).is_some();

			if idx == end_range {
				idx = start_range;
			}

			idx += 1;
		});
	});

	c.bench_function("cache_get_bytes", |b| {
		let n = 64;
		let mut cache = Cache::with_size_kb(n as usize);

		let start_range = (n as f32 * 0.25).round() as u32;
		let end_range = (n as f32 * 0.75).round() as u32;

		let mut idx = start_range;

		populate(&mut cache, n as usize, 1024);

		b.iter(|| {
			let _ = cache.get_bytes(format!("key-{}", idx)).is_some();

			if idx == end_range {
				idx = start_range;
			}

			idx += 1;
		});
	});

	c.bench_function("cache_get_bytes_1mb", |b| {
		let n = 1024;
		let mut cache = Cache::with_size_kb(n as usize);

		let start_range = (n as f32 * 0.25).round() as u32;
		let end_range = (n as f32 * 0.75).round() as u32;

		let mut idx = start_range;

		populate(&mut cache, n as usize, 1024);

		b.iter(|| {
			let _ = cache.get_bytes(format!("key-{}", idx)).is_some();

			if idx == end_range {
				idx = start_range;
			}

			idx += 1;
		});
	});

	c.bench_function("cache_get_bytes_4mb", |b| {
		let n = 4 * 1024;
		let mut cache = Cache::with_size_kb(n as usize);

		let start_range = (n as f32 * 0.25).round() as u32;
		let end_range = (n as f32 * 0.75).round() as u32;

		let mut idx = start_range;

		populate(&mut cache, n as usize, 1024);

		b.iter(|| {
			let _ = cache.get_bytes(format!("key-{}", idx)).is_some();

			if idx == end_range {
				idx = start_range;
			}

			idx += 1;
		});
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
