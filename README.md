# in-memory-cache

A high-performance, thread-safe in-memory cache implementation in Rust with LRU (Least Recently Used) eviction policy. This library provides flexible caching strategies with support for both capacity-based and memory size-based limits.

## Features

- **LRU Eviction Policy**: Automatically removes least recently used items when limits are reached
- **Flexible Limiting Strategies**: 
  - Capacity-based: Limit by number of items
  - Size-based: Limit by total memory usage (bytes, KB, MB, GB)
- **Generic Key-Value Storage**: Keys are converted to strings, values stored as bytes
- **Zero-Copy Operations**: Efficient memory usage with `bytes::Bytes`
- **Simple API**: Easy to use with intuitive method names

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
in-memory-cache = "0.4.0"
```

## Usage

### Basic Example with Capacity Limit

```rust
use in_memory_cache::Cache;

fn main() {
    // Create a cache that holds up to 100 items
    let mut cache = Cache::with_capacity(100);
    
    // Add some data
    cache.add("user:1", b"John Doe".to_vec()).unwrap();
    cache.add("user:2", b"Jane Smith".to_vec()).unwrap();
    
    // Retrieve data
    if let Some(entry) = cache.get("user:1") {
        println!("Found user: {:?}", entry);
    }
    
    // Get just the bytes
    if let Some(data) = cache.get_bytes("user:2") {
        println!("User data: {:?}", data);
    }
}
```

### Size-Based Caching

```rust
use in_memory_cache::Cache;

fn main() {
    // Create a cache limited to 1MB of data
    let mut cache = Cache::with_size_mb(1);
    
    // Add large data items
    let large_data = vec![0u8; 512 * 1024]; // 512KB
    cache.add("large_item_1", large_data).unwrap();
    
    let another_large_data = vec![1u8; 512 * 1024]; // 512KB  
    cache.add("large_item_2", another_large_data).unwrap();
    
    // This will evict the first item due to size limit
    let third_item = vec![2u8; 100 * 1024]; // 100KB
    cache.add("large_item_3", third_item).unwrap();
    
    // large_item_1 is now evicted
    assert!(cache.get("large_item_1").is_none());
    assert!(cache.get("large_item_2").is_some());
    assert!(cache.get("large_item_3").is_some());
}
```

### Available Cache Creation Methods

```rust
use in_memory_cache::Cache;

// Capacity-based (number of items)
let cache1 = Cache::with_capacity(1000);

// Size-based (bytes)
let cache2 = Cache::with_size(1024 * 1024); // 1MB
let cache3 = Cache::with_size_kb(1024);     // 1MB  
let cache4 = Cache::with_size_mb(1);        // 1MB
let cache5 = Cache::with_size_gb(1);        // 1GB
```

### API Overview

- `Cache::with_capacity(n)` - Create cache limited by number of items
- `Cache::with_size(bytes)` - Create cache limited by total size in bytes
- `Cache::with_size_kb(kb)` - Create cache limited by size in kilobytes
- `Cache::with_size_mb(mb)` - Create cache limited by size in megabytes  
- `Cache::with_size_gb(gb)` - Create cache limited by size in gigabytes
- `add(key, value)` - Add/update an item in the cache
- `get(key)` - Retrieve an entry (moves to front of LRU)
- `get_bytes(key)` - Retrieve just the value bytes
- `clear()` - Remove all items from the cache

## Benchmarks

To run performance benchmarks:

```bash
cargo bench
```

To run the included example with performance measurements:

```bash
cargo run --example basics
```

## Examples

Check out the `examples/` directory for more detailed usage examples:

```bash
cargo run --example basics
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Clone the repository
2. Install Rust (if not already installed): https://rustup.rs/
3. Run tests: `cargo test`
4. Run benchmarks: `cargo bench`
5. Format code: `cargo fmt`

### Guidelines

- Write tests for new features
- Ensure all tests pass before submitting PR
- Follow existing code style and formatting
- Update documentation for API changes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Đorđe Zeljić <zeljic@gmail.com>
