# in-memory-cache
A Rust library providing a naive implementation of an in-memory cache with LRU (Least Recently Used) eviction. The cache supports both capacity-based (number of items) and size-based (memory usage) limits.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Environment Setup
- Rust 1.89.0+ and Cargo are pre-installed and ready to use
- No additional dependencies or SDKs need to be installed

### Build and Test Commands
- `cargo build` -- first build takes ~12 seconds (downloads deps), subsequent builds ~0.5 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
- `cargo build --release` -- optimized build takes ~0.5 seconds after dependencies are built
- `cargo test` -- first test run takes ~12 seconds (downloads criterion deps), subsequent test runs <0.2 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
- `cargo clippy` -- lint check takes ~0.5 seconds  
- `cargo fmt --check` -- format check takes ~0.2 seconds
- `cargo doc` -- documentation generation takes ~1.3 seconds

### Benchmarks
- `cargo bench` -- benchmarks take ~1.5 minutes to complete. NEVER CANCEL. Set timeout to 120+ seconds.
  - Runs criterion-based performance benchmarks for cache operations
  - Tests cache_add, cache_get, cache_get_bytes with various data sizes (64KB to 4MB)
  - Outputs detailed performance metrics with statistical analysis

### Examples  
- `cargo run --example basics` -- performance demonstration takes ~2 seconds
  - Runs cache performance tests showing capacity vs size-based caching
  - Outputs timing data for different cache configurations and access patterns

## Validation

### Manual Testing
- Always test cache functionality after making changes to core library code
- The cache implements LRU eviction with two modes:
  - **Capacity-based**: `Cache::with_capacity(n)` - limits number of items
  - **Size-based**: `Cache::with_size(bytes)` or helpers like `with_size_kb(kb)`, `with_size_mb(mb)`
- Core operations: `add(key, value)`, `get(key)`, `get_bytes(key)`, `clear()`
- Test both successful operations and error conditions (oversized items)

### Critical Validation Scenarios
After making changes, always test these key behaviors:
1. **LRU Eviction**: Create capacity-2 cache, add 3 items, verify oldest is evicted
2. **Size Limits**: Create 1KB cache, verify 2KB item is rejected with error
3. **Data Retrieval**: Verify `get_bytes()` returns correct data and length
4. **Cache Clearing**: Verify `clear()` removes all items
5. **LRU Reordering**: Access middle item, verify it moves to front of cache

Example validation:
```rust
let mut cache = Cache::with_capacity(2);
cache.add("key1", b"value1".to_vec()).unwrap();
cache.add("key2", b"value2".to_vec()).unwrap();
cache.add("key3", b"value3".to_vec()).unwrap();  // Should evict key1
assert!(cache.get("key1").is_none());  // key1 evicted
assert!(cache.get("key2").is_some());  // key2 still present
```

### Required Validation Steps
- Always run `cargo test` after making code changes
- Run `cargo clippy` to check for Rust best practices violations  
- Run `cargo fmt --check` to ensure code formatting compliance
- For performance-related changes, run `cargo bench` to measure impact

### Making Changes to This Codebase
- The main library code is in `src/lib.rs` (~285 lines total)
- Tests are embedded in the same file using `#[cfg(test)]` blocks
- Add new functionality to the `Cache` impl block
- When adding new methods, follow the existing pattern using generic types `T: Into<String>` for keys
- Always add corresponding unit tests for new functionality
- Benchmark new operations by adding to `benches/basic_bench.rs` if performance-critical

## Common Tasks

### Repository Structure
```
/home/runner/work/in-memory-cache/in-memory-cache/
├── .git/
├── .gitignore         -- ignores target/, Cargo.lock, .idea/
├── .rustfmt.toml      -- Rust formatting config (edition=2021, hard_tabs=true, max_width=128)
├── Cargo.toml         -- main project file with criterion benchmarks
├── Cargo.lock         -- generated after first build
├── LICENSE            -- MIT license
├── README.md          -- basic project description with benchmark command
├── src/
│   └── lib.rs         -- main library implementation (~285 lines)
├── examples/
│   └── basics.rs      -- performance demonstration example  
├── benches/
│   └── basic_bench.rs -- criterion-based benchmarks
├── perf/              -- performance log files
└── target/            -- build artifacts (gitignored)
```

### Key Library Components (src/lib.rs)
- `struct Cache` -- main cache implementation with VecDeque storage
- `struct Entry` -- cache entry with key (String) and value (bytes::Bytes)
- `enum LimitType` -- Capacity vs Size limiting modes
- Public API:
  - Constructors: `with_capacity()`, `with_size()`, `with_size_kb()`, `with_size_mb()`, `with_size_gb()`
  - Operations: `add()`, `get()`, `get_bytes()`, `clear()`
- Built-in tests: 4 unit tests covering basic operations, capacity limits, clearing, and overflow handling

### Dependencies (Cargo.toml)
- Runtime: `bytes = "1.4"` for efficient byte handling
- Development: `criterion = "0.5"` for benchmarking (pulls ~40 additional deps)

### Common Output Patterns
When tests pass:
```
running 4 tests
test tests::clear ... ok
test tests::overflow ... ok  
test tests::with_capacity_by_key ... ok
test tests::with_capacity_basic_operations ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

When benchmarks complete successfully, you'll see:
```
cache_add               time:   [6.0313 µs 6.5601 µs 7.0064 µs]
cache_get               time:   [185.36 ns 186.93 ns 188.39 ns]
cache_get_bytes         time:   [187.56 ns 189.73 ns 191.89 ns]
cache_get_bytes_1mb     time:   [2.4356 µs 2.4533 µs 2.4710 µs]  
cache_get_bytes_4mb     time:   [10.453 µs 10.562 µs 10.692 µs]
```

### Performance Characteristics
- GET operations: ~185-190 nanoseconds for small caches
- GET operations scale with data size: ~2.5µs for 1MB items, ~10.5µs for 4MB items  
- ADD operations: ~6-7 microseconds
- LRU reordering happens automatically on cache hits (moves item to front)
- Size-based caches reject items larger than the total cache limit