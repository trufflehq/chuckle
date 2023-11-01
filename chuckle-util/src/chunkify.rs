use vesper::prelude::Parse;

#[derive(Parse, Debug)]
pub enum RemainderStrategy {
	Overflow,
	Exclude,
}

pub struct ChunkRemainder<T> {
	pub chunks: Vec<Vec<T>>,
	pub excluded: Vec<T>,
}

impl<T> ChunkRemainder<T> {
	fn new(chunks: Vec<Vec<T>>, excluded: Vec<T>) -> Self {
		Self { chunks, excluded }
	}
}

pub fn chunkify<T: Clone>(
	vec: Vec<T>,
	chunk_size: usize,
	strategy: RemainderStrategy,
) -> ChunkRemainder<T> {
	if chunk_size == 0 {
		panic!("chunk_size must be greater than 0");
	}

	let mut chunks = vec
		.iter()
		.enumerate()
		.map(|(i, item)| (i / chunk_size, item.clone()))
		.fold(
			vec![Vec::new(); vec.len() / chunk_size + 1],
			|mut acc, (idx, item)| {
				acc[idx].push(item);
				acc
			},
		);

	match strategy {
		RemainderStrategy::Overflow => {
			let overflow_items = chunks.pop().unwrap_or_default();
			let mut overflow_iter = overflow_items.into_iter().rev();
			for chunk in chunks.iter_mut().rev() {
				if let Some(value) = overflow_iter.next() {
					chunk.push(value);
				} else {
					break;
				}
			}
			ChunkRemainder::new(chunks, Vec::new())
		}
		RemainderStrategy::Exclude => {
			let remainder = chunks.pop().unwrap_or_default();
			ChunkRemainder::new(chunks, remainder)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_overflow() {
		let v = vec![1, 2, 3, 4, 5, 6, 7];
		let result = chunkify(v, 3, RemainderStrategy::Overflow);
		assert_eq!(result.chunks, vec![vec![1, 2, 3], vec![4, 5, 6, 7]]);
		assert_eq!(result.excluded, Vec::<i32>::new());
	}

	fn irange(start: i32, stop: i32) -> Vec<i32> {
		(start..=stop).collect::<Vec<i32>>()
	}

	#[test]
	fn test_overflow_big() {
		// create a vec of usizes going from 1 to 35
		let v = irange(1, 35);
		let result = chunkify(v, 6, RemainderStrategy::Overflow);
		assert_eq!(
			result.chunks,
			vec![
				vec![1, 2, 3, 4, 5, 6, 31],
				vec![7, 8, 9, 10, 11, 12, 32],
				vec![13, 14, 15, 16, 17, 18, 33],
				vec![19, 20, 21, 22, 23, 24, 34],
				vec![25, 26, 27, 28, 29, 30, 35],
			]
		);
		assert_eq!(result.excluded, Vec::<i32>::new());
	}

	#[test]
	fn test_exclude() {
		let v = vec![1, 2, 3, 4, 5, 6, 7];
		let result = chunkify(v, 3, RemainderStrategy::Exclude);
		assert_eq!(result.chunks, vec![vec![1, 2, 3], vec![4, 5, 6]]);
		assert_eq!(result.excluded, vec![7]);
	}

	#[test]
	#[should_panic(expected = "chunk_size must be greater than 0")]
	fn test_invalid_chunk_size() {
		let v = vec![1, 2, 3, 4, 5, 6, 7];
		chunkify(v, 0, RemainderStrategy::Exclude);
	}
}
