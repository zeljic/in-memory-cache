# In-Memory-Cache

This crate provides an implementation of an in-memory cache in Rust using LRU Eviction Strategy.

It can store entries with set capacity or a set size limit. Each entry has a key of type string and a value byte type.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.in-memory-cache]
version = "0.4.0"
git = "https://github.com/zeljic/in-memory-cache.git"
```

## Examples

Here is a sample code of the usage:

```rust,no_run
fn main() {
    // Import the crate
    use in_memory_cache::*;

    // Create an instance of cache with capacity limit
    let mut cache_with_capacity = Cache::with_capacity(2);

    // Add items to the cache
    assert_eq!(cache_with_capacity.add("k1", "v1").is_ok(), true);
    assert_eq!(cache_with_capacity.add("k2", "v2").is_ok(), true);
    assert_eq!(cache_with_capacity.add("k3", "v3").is_ok(), true); // since limit is 2, this will remove the "k1"-"v1" entry

    // Create an instance of cache with size limit 
    let mut cache_with_size = Cache::with_size(20); // size limit is 20 bytes

    // Add items to the cache with size limit
    assert_eq!(cache_with_size.add("k1", "v1").is_ok(), true);
    assert_eq!(cache_with_size.add("k2", "v2").is_ok(), true); 
    assert_eq!(cache_with_size.add("k3", "v3").is_ok(), true); // this will pass since total size doesn't exceed 20 bytes
    assert_eq!(cache_with_size.add("k4", "value4").is_ok(), true); // this will remove "k1"-"v1" since total size would exceed 20 bytes

    assert_eq!(cache_with_size.add("k5", "value that exceeds the limit").is_err(), true); // this will fail since size of value is greater than limit

    // Fetch an entry based on the key
    let entry = cache_with_capacity.get("k1");
    assert_eq!(entry.unwrap().key, "k1");

    // Clear all entries in the cache
    cache_with_capacity.clear();
    assert_eq!(cache_with_capacity.get("k1").is_none(), true);
}
```

## Error Handling

The `add()` function returns a `Result`. If the size of the value exceeds the size limit of the cache then an error of variant `Error` is returned with error message "Content is too big: actual_size > limit_size". The error can be caught and handled as follows:

```rust,no_run
match cache_with_size.add("k5", "value that exceeds the limit") {
    Ok(_) => println!("Added successfully."),
    Err(e) => println!("Failed to add. Error: {}", e),
}
```

## LRU Eviction Strategy

The cache uses LRU (Least Recently Used) eviction strategy. This means, when the cache reaches its capacity or size limit and more space is required for a new entry, the least recently used entry gets evicted. 

This strategy is implemented using a `VecDeque`. Every time an item is accessed, it is moved to the front of the queue. When eviction is necessary, the item at the end of the queue (which will be the least recently used item) is removed.
