use crate::Result;
use rust_decimal::Decimal as RustDecimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Decimal {
	I32([u8; 4]),
	I64([u8; 8]),
	I128([u8; 16]),
	I256([u8; 32]),
}

macro_rules! create_decimal_from_byte_array {
	($ty: ty, $variant: ident) => {
		impl From<$ty> for Decimal {
			fn from(value: $ty) -> Self {
				Decimal::$variant(value)
			}
		}
	};
}

create_decimal_from_byte_array!([u8; 4], I32);
create_decimal_from_byte_array!([u8; 8], I64);
create_decimal_from_byte_array!([u8; 16], I128);
create_decimal_from_byte_array!([u8; 32], I256);

impl Decimal {
	pub fn try_to_rust_decimal(&self, scale: u32) -> Result<RustDecimal> {
		match *self {
			Decimal::I32(data) => {
				let num = i32::from_le_bytes(data);
				Ok(RustDecimal::new(num as i64, scale))
			}
			Decimal::I64(data) => {
				let num = i64::from_le_bytes(data);
				Ok(RustDecimal::new(num, scale))
			}
			Decimal::I128(data) => {
				let num = i128::from_le_bytes(data);
				Ok(RustDecimal::try_from_i128_with_scale(num, scale)?)
			}
			Decimal::I256(data) => {
				// return error if workflow.
				if data[16..].iter().any(|num| *num > 0) {
					return Err(rust_decimal::Error::ErrorString("Exceeds the range of decimal".to_owned()))?;
				}

				let num = i128::from_le_bytes(data[..16].try_into().unwrap());
				Ok(RustDecimal::try_from_i128_with_scale(num, scale)?)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Decimal;
	use super::RustDecimal;

	#[test]
	fn test_decimal() {
		let decimal32 = Decimal::I32(i32::MAX.to_le_bytes());
		assert_eq!(RustDecimal::try_new(i32::MAX as i64, 2).ok(), decimal32.try_to_rust_decimal(2).ok());

		let mut arr = [0u8; 32];
		arr[..16].copy_from_slice(i128::MAX.to_le_bytes().as_slice());
		let mut decimal256 = Decimal::I256(arr);
		assert_eq!(None, decimal256.try_to_rust_decimal(2).ok());

		arr[12..16].iter_mut().for_each(|num| *num = 0);
		decimal256 = Decimal::I256(arr);
		let mut expect: RustDecimal = RustDecimal::MAX;
		let _ = expect.set_scale(2);
		assert_eq!(Some(expect), decimal256.try_to_rust_decimal(2).ok());
	}
}
