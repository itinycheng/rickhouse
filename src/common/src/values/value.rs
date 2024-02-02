use crate::values::ip::IpV4;
use crate::values::ip::IpV6;
use std::collections::HashMap;

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
