use serde::Deserialize;
use serde::Serialize;

use crate::values::ip::IpV4;
use crate::values::ip::IpV6;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize)]
pub enum Value {
	Null,
	Bool(bool),
	Int8(i8),
	Int16(i16),
	Int32(i32),
	Int64(i64),
	Int128(i128),
	#[cfg(feature = "bigint")]
	Int256(crate::values::bigint::i256),
	UInt8(u8),
	UInt16(u16),
	UInt32(u32),
	UInt64(u64),
	UInt128(u128),
	#[cfg(feature = "bigint")]
	UInt256(crate::values::bigint::u256),
	Float32(f32),
	Float64(f64),
	String(Vec<u8>),
	Ipv4(IpV4),
	Ipv6(IpV6),
	#[cfg(feature = "uuid")]
	Uuid(crate::values::uuid::Uuid),
	Date(u16),
	Date32(i32),
	DateTime(u32),
	DateTime64(u64),
	#[cfg(feature = "decimal")]
	Decimal(crate::values::decimal::Decimal),
	Enum8(Vec<Value>),
	Enum16(Vec<Value>),
	#[cfg(feature = "json")]
	Json(crate::values::json::Json),
	Tuple(Vec<Value>),
	Array(Vec<Value>),
	Map(HashMap<Value, Value>),
}

impl Hash for Value {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::String(s) => s.hash(state),
			Self::Int8(i) => i.hash(state),
			Self::Int16(i) => i.hash(state),
			Self::Int32(i) => i.hash(state),
			Self::Int64(i) => i.hash(state),
			Self::Int128(i) => i.hash(state),
			Self::Int256(i) => i.hash(state),
			Self::UInt8(i) => i.hash(state),
			Self::UInt16(i) => i.hash(state),
			Self::UInt32(i) => i.hash(state),
			Self::UInt64(i) => i.hash(state),
			Self::UInt128(i) => i.hash(state),
			Self::UInt256(i) => i.hash(state),
			Self::Uuid(i) => i.hash(state),
			Self::Date(i) => i.hash(state),
			Self::Date32(i) => i.hash(state),
			Self::DateTime(i) => i.hash(state),
			Self::DateTime64(i) => i.hash(state),
			Self::Enum8(i) => i.hash(state),
			Self::Enum16(i) => i.hash(state),
			_ => unimplemented!("Hash trait unimplemented"),
		}
	}
}

impl Eq for Value {}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Bool(l0), Self::Bool(r0)) => *l0 == *r0,
			(Self::Int8(l0), Self::Int8(r0)) => *l0 == *r0,
			(Self::Int16(l0), Self::Int16(r0)) => *l0 == *r0,
			(Self::Int32(l0), Self::Int32(r0)) => *l0 == *r0,
			(Self::Int64(l0), Self::Int64(r0)) => *l0 == *r0,
			(Self::Int128(l0), Self::Int128(r0)) => *l0 == *r0,
			(Self::Int256(l0), Self::Int256(r0)) => *l0 == *r0,
			(Self::UInt8(l0), Self::UInt8(r0)) => *l0 == *r0,
			(Self::UInt16(l0), Self::UInt16(r0)) => *l0 == *r0,
			(Self::UInt32(l0), Self::UInt32(r0)) => *l0 == *r0,
			(Self::UInt64(l0), Self::UInt64(r0)) => *l0 == *r0,
			(Self::UInt128(l0), Self::UInt128(r0)) => *l0 == *r0,
			(Self::UInt256(l0), Self::UInt256(r0)) => *l0 == *r0,
			(Self::Float32(l0), Self::Float32(r0)) => *l0 == *r0,
			(Self::Float64(l0), Self::Float64(r0)) => *l0 == *r0,
			(Self::String(l0), Self::String(r0)) => *l0 == *r0,
			(Self::Ipv4(l0), Self::Ipv4(r0)) => *l0 == *r0,
			(Self::Ipv6(l0), Self::Ipv6(r0)) => *l0 == *r0,
			(Self::Uuid(l0), Self::Uuid(r0)) => *l0 == *r0,
			(Self::Date(l0), Self::Date(r0)) => *l0 == *r0,
			(Self::Date32(l0), Self::Date32(r0)) => *l0 == *r0,
			(Self::DateTime(l0), Self::DateTime(r0)) => *l0 == *r0,
			(Self::DateTime64(l0), Self::DateTime64(r0)) => *l0 == *r0,
			(Self::Decimal(l0), Self::Decimal(r0)) => *l0 == *r0,
			(Self::Enum8(l0), Self::Enum8(r0)) => *l0 == *r0,
			(Self::Enum16(l0), Self::Enum16(r0)) => *l0 == *r0,
			(Self::Json(l0), Self::Json(r0)) => *l0 == *r0,
			(Self::Tuple(l0), Self::Tuple(r0)) => *l0 == *r0,
			(Self::Array(l0), Self::Array(r0)) => *l0 == *r0,
			_ => false,
		}
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn test() {
		assert_eq!(1, 1);
	}
}
