use std::{
	collections::VecDeque,
	fmt::{Debug, Formatter},
};

#[derive(Clone)]
pub struct Entry {
	key: String,
	value: bytes::Bytes,
}

impl Entry {
	pub fn new<T, V>(key: T, value: V) -> Self
	where
		T: Into<String>,
		V: Into<bytes::Bytes>,
	{
		Self {
			key: key.into(),
			value: value.into(),
		}
	}
}

impl<T: Into<String>, V: Into<bytes::Bytes>> From<(T, V)> for Entry {
	fn from((key, value): (T, V)) -> Self {
		let value = value.into();

		Entry { key: key.into(), value }
	}
}

impl Debug for Entry {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Entry")
			.field("key", &self.key)
			.field("size", &self.value.len())
			.finish()
	}
}

#[derive(Debug, PartialEq)]
enum LimitType {
	Capacity,
	Size,
}

pub struct Cache {
	items: VecDeque<Entry>,
	limit_type: LimitType,
	limit: usize,
}

impl Debug for Cache {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut debug_struct = f.debug_struct("Cache");

		debug_struct
			.field("items", &self.items.len())
			.field("limit_type", &self.limit_type)
			.field("limit", &self.limit);

		match self.limit_type {
			LimitType::Capacity => {
				debug_struct.field("capacity", &self.len_capacity());
			}
			LimitType::Size => {
				debug_struct.field("size", &self.len_size());
			}
		}

		debug_struct.finish()
	}
}

impl Cache {
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			items: VecDeque::with_capacity(capacity),
			limit_type: LimitType::Capacity,
			limit: capacity,
		}
	}

	pub fn with_size(size: usize) -> Self {
		Self {
			items: VecDeque::new(),
			limit_type: LimitType::Size,
			limit: size,
		}
	}

	pub fn with_size_kb(size: usize) -> Self {
		Self::with_size(size * 1024)
	}

	pub fn with_size_mb(size: usize) -> Self {
		Self::with_size_kb(size * 1024)
	}

	pub fn with_size_gb(size: usize) -> Self {
		Self::with_size_mb(size * 1024)
	}

	pub fn add<T, V>(&mut self, key: T, value: V) -> anyhow::Result<()>
	where
		T: Into<String>,
		V: Into<bytes::Bytes>,
	{
		let key: String = key.into();
		let value: bytes::Bytes = value.into();

		if !self.items.iter().any(|entry| entry.key == key) {
			match self.limit_type {
				LimitType::Capacity => {
					let len = self.items.len();

					if len == self.limit {
						self.items.remove(len - 1);
					}
				}
				LimitType::Size => {
					if value.len() > self.limit {
						return Err(anyhow::Error::msg("Overflow"));
					}

					while self.len_size() + value.len() > self.limit {
						self.items.remove(self.items.len() - 1);
					}
				}
			}

			self.items.push_front(Entry::new(key, value));
		}

		Ok(())
	}

	pub fn get<T>(&mut self, key: T) -> Option<Entry>
	where
		T: Into<String>,
	{
		let key: String = key.into();

		if let Some(position) = self.items.iter().position(|entry| entry.key == key) {
			if position == 0 {
				if let Some(entry) = self.items.get(0) {
					return Some(entry.to_owned());
				}
			} else if let Some(entry) = self.items.remove(position) {
				self.items.push_front(entry.to_owned());

				return Some(entry);
			}
		}

		None
	}

	pub fn get_bytes<T>(&mut self, key: T) -> Option<bytes::Bytes>
	where
		T: Into<String>,
	{
		let key: String = key.into();

		if let Some(position) = self.items.iter().position(|entry| entry.key == key) {
			if position == 0 {
				if let Some(entry) = self.items.get(0) {
					return Some(entry.value.to_owned());
				}
			} else if let Some(entry) = self.items.remove(position) {
				self.items.push_front(entry.to_owned());

				return Some(entry.value);
			}
		}

		None
	}

	pub fn clear(&mut self) {
		self.items.clear();
	}

	fn len_capacity(&self) -> usize {
		self.items.len()
	}

	fn len_size(&self) -> usize {
		self.items.iter().fold(0_usize, |sum, entry| sum + entry.value.len())
	}
}

#[cfg(test)]
mod tests {
	use crate::Cache;

	fn populate(cache: &mut Cache, size: usize, prefix: &str) {
		for i in 0..cache.limit {
			let v = vec![1; size];

			if cache.add(format!("{}{}", prefix, i), v).is_ok() {}
		}
	}

	#[test]
	fn simple() {
		let mut cache = Cache::with_capacity(4);

		populate(&mut cache, 512, "key_");

		assert_eq!("key_2", cache.items.get(1).unwrap().key);
	}
}
