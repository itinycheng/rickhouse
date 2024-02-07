#![allow(dead_code)]

mod error;
pub mod metadata;
mod serde;
pub mod values;

pub use error::Error;
pub use error::Result;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
