#![allow(non_camel_case_types)]

macro_rules! impl_num_256 {
	($name: ident, $to_ty: ty, $to_fn: ident) => {
		#[derive(::serde::Serialize, ::serde::Deserialize, Hash, PartialEq, Eq)]
		pub struct $name(pub [u8; 32]);

		impl $name {
			pub fn new(data: [u8; 32]) -> Self {
				Self(data)
			}
		}

		impl From<$name> for $to_ty {
			fn from(value: $name) -> Self {
				Self::$to_fn(value.0.as_slice())
			}
		}
	};
}

impl_num_256!(u256, num_bigint::BigUint, from_bytes_le);
impl_num_256!(i256, num_bigint::BigInt, from_signed_bytes_le);

#[cfg(test)]
mod tests {
	use super::i256;
	use num_bigint::BigInt;
	use rust_decimal::prelude::FromPrimitive;

	#[test]
	fn test_bigint() {
		let mut arr = [0u8; 32];
		arr[..16].copy_from_slice(u128::MAX.to_le_bytes().as_slice());
		let big_int: BigInt = i256::new(arr).into();
		assert_eq!(BigInt::from_u128(u128::MAX), Some(big_int))
	}
}
