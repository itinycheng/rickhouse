use bytes::Buf;
use std::collections::VecDeque;

use crate::Result;

#[derive(Default)]
pub(crate) struct PartBuf<T> {
	buffers: VecDeque<T>,
	remain: usize,
}

impl<T: Buf> PartBuf<T> {
	pub fn new() -> Self {
		PartBuf { buffers: VecDeque::new(), remain: 0 }
	}

	pub fn push_back(&mut self, t: T) {
		let size = t.remaining();

		if size > 0 {
			self.buffers.push_back(t);
			self.remain += size;
		}
	}

	pub fn pop_front(&mut self) -> Option<T> {
		match self.buffers.pop_front() {
			Some(t) => {
				let size = t.remaining();
				self.remain -= size;
				Some(t)
			}
			None => None,
		}
	}
}

impl<T: Buf> Buf for PartBuf<T> {
	fn remaining(&self) -> usize {
		self.remain
	}

	fn chunk(&self) -> &[u8] {
		if let Some(head) = self.buffers.front() {
			head.chunk()
		} else {
			&[]
		}
	}

	fn advance(&mut self, cnt: usize) {
		self.remain -= cnt;

		// advance head and remove if it empty.
		if let Some(head) = self.buffers.front_mut() {
			head.advance(cnt);
			if head.remaining() == 0 {
				self.buffers.pop_front();
			}
		}
	}
}

pub(crate) trait BufExp: Buf {
	fn ensure_size(&self, size: usize) -> Result<()>;
	fn read_size(&mut self) -> Result<usize>;
	fn read_vec(&mut self, size: usize) -> Result<Vec<u8>>;
	fn read_u64_leb128(&mut self) -> Result<u64>;
	fn read_utf8_string(&mut self) -> Result<String>;
}

impl<T: Buf> BufExp for PartBuf<T> {
	#[inline]
	fn ensure_size(&self, size: usize) -> Result<()> {
		match size >= self.remaining() {
			true => Ok(()),
			false => Err(crate::Error::NotEnoughData),
		}
	}

	#[inline]
	fn read_size(&mut self) -> Result<usize> {
		 Ok(self.read_u64_leb128()?.try_into()?)
	}

	#[inline]
	fn read_vec(&mut self, size: usize) -> Result<Vec<u8>> {
		self.ensure_size(size)?;
		let mut vec = vec![0; size];
		self.copy_to_slice(&mut vec[..]);
		Ok(vec)
	}

	#[inline]
	fn read_u64_leb128(&mut self) -> Result<u64> {
		let mut arr = [0u8; 8];

		let mut idx = 0;
		loop {
			self.ensure_size(1)?;
			let byte = self.get_u8();
			arr[idx] = byte;
			idx += 1;
			if byte & 0x80 == 0 {
				break;
			}
		}

		Ok(leb128::read::unsigned(&mut &arr[..])?)
	}

	fn read_utf8_string(&mut self) -> Result<String> {
		let size = self.read_size()?;
		let vec = self.read_vec(size)?;
		Ok(String::from_utf8(vec).map_err(|err| err.utf8_error())?)
	}
}

#[cfg(test)]
mod tests {
	use bytes::Buf;
	use bytes::Bytes;

	use super::PartBuf;

	#[test]
	fn test() {
		let mut buf = PartBuf::new();

		buf.push_back(Bytes::from_static(&[1]));
		buf.push_back(Bytes::from_static(&[]));
		buf.push_back(Bytes::from_static(&[2, 3]));
		buf.push_back(Bytes::from_static(&[4, 5]));

		assert_eq!(u16::from_le_bytes([1, 2]), buf.get_u16_le());
		assert_eq!(u8::from_le_bytes([3]), buf.get_u8());
		assert_eq!(i16::from_le_bytes([4, 5]), buf.get_i16_le());

		test_mut_ref(&mut buf)
	}

	fn test_mut_ref<T: Buf>(t: T) {
		assert_eq!(0, t.remaining());
	}
}
