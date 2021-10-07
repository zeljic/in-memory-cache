use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Entry {
	key: String,
	value: Vec<u8>,
}

impl Entry {
	pub fn new<T>(key: T, value: Vec<u8>) -> Self
	where
		T: Into<String>,
	{
		Self { key: key.into(), value }
	}
}

impl<T: Into<String>, V: Into<Vec<u8>>> From<(T, V)> for Entry {
	fn from((key, value): (T, V)) -> Self {
		Entry {
			key: key.into(),
			value: value.into(),
		}
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

#[derive(Debug)]
pub struct Cache {
	items: VecDeque<Entry>,
	capacity: usize,
}

impl Cache {
	pub fn new(capacity: usize) -> Self {
		Self {
			items: VecDeque::with_capacity(capacity),
			capacity,
		}
	}

	pub fn add<T, V>(&mut self, key: T, value: V)
	where
		T: Into<String>,
		V: Into<Vec<u8>>,
	{
		let key: String = key.into();

		if !self.items.iter().any(|entry| entry.key == key) {
			self.items.push_front(Entry::new(key, value.into()));
		}
	}

	pub fn get<T>(&mut self, key: T) -> Option<Entry>
	where
		T: Into<String>,
	{
		let key: String = key.into();

		if let Some(position) = self.items.iter().position(|entry| entry.key == key) {
			return if let Some(entry) = self.items.remove(position) {
				self.items.push_front(entry.to_owned());

				Some(entry)
			} else {
				None
			};
		}

		None
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

#[cfg(test)]
mod tests {
	use crate::Cache;

	#[test]
	fn it_works() {
		let mut cache = Cache::new(32);

		cache.add("empty", vec![]);
		cache.add("hi", String::from("zdravo svete").as_bytes().to_vec());

		cache.get("hi");

		assert_eq!("hi", cache.items.get(0).unwrap().key);
	}
}
