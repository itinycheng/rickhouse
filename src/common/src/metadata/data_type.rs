use std::str::FromStr;

use crate::error::Error;
use crate::error::Result;

use super::type_util::parse_agg_func;
use super::type_util::parse_enum;
use super::type_util::parse_map;
use super::type_util::parse_tuple;

/// data type.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DataType {
	Bool,
	Int8,
	Int16,
	Int32,
	Int64,
	Int128,
	Int256,
	UInt8,
	UInt16,
	UInt32,
	UInt64,
	UInt128,
	UInt256,
	Float32,
	Float64,
	String,
	FixedString(usize),
	Ipv4,
	Ipv6,
	Uuid,
	Date,
	Date32,
	DateTime(Option<Tz>),
	DateTime64(u8, Option<Tz>),
	Decimal(u8, u8),
	Decimal32(u8),
	Decimal64(u8),
	Decimal128(u8),
	Decimal256(u8),
	Enum8(Vec<(String, i8)>),
	Enum16(Vec<(String, i16)>),
	LowCardinality(Box<DataType>),
	AggregateFunction(AggFunc, Vec<DataType>),
	SimpleAggregateFunction(AggFunc, Vec<DataType>),
	Json,
	Tuple(Vec<(String, DataType)>),
	Array(Box<DataType>),
	Map(Box<DataType>, Box<DataType>),
	Nullable(Box<DataType>),
}

impl FromStr for DataType {
	type Err = crate::error::Error;
	fn from_str(s: &str) -> Result<Self> {
		Ok(match s.trim() {
			"Bool" => DataType::Bool,
			"Int8" => DataType::Int8,
			"Int16" => DataType::Int16,
			"Int32" => DataType::Int32,
			"Int64" => DataType::Int64,
			"Int128" => DataType::Int128,
			"Int256" => DataType::Int256,
			"UInt8" => DataType::UInt8,
			"UInt16" => DataType::UInt16,
			"UInt32" => DataType::UInt32,
			"UInt64" => DataType::UInt64,
			"UInt128" => DataType::UInt128,
			"UInt256" => DataType::UInt256,
			"Float32" => DataType::Float32,
			"Float64" => DataType::Float64,
			"String" => DataType::String,
			s if s.starts_with("FixedString") => DataType::FixedString(s[12..(s.len() - 1)].parse()?),
			"IPv4" => DataType::Ipv4,
			"IPv6" => DataType::Ipv6,
			"UUID" => DataType::Uuid,
			"Date" => DataType::Date,
			"Date32" => DataType::Date32,
			// decimal.
			s if s.starts_with("Decimal(") => {
				let sub_str = &s[8..(s.len() - 1)];
				let idx = sub_str.find(',').ok_or(Error::ParseTypeError(format!("can't parse decimal type: {}", s)))?;
				DataType::Decimal(sub_str[..idx].trim().parse()?, sub_str[(idx + 1)..].trim().parse()?)
			}
			s if s.starts_with("Decimal32") => DataType::Decimal32(s[10..(s.len() - 1)].parse()?),
			s if s.starts_with("Decimal64") => DataType::Decimal64(s[10..(s.len() - 1)].parse()?),
			s if s.starts_with("Decimal128") => DataType::Decimal128(s[11..(s.len() - 1)].parse()?),
			s if s.starts_with("Decimal256") => DataType::Decimal256(s[11..(s.len() - 1)].parse()?),
			// dateTime.
			"DateTime" => DataType::DateTime(None),
			s if s.starts_with("DateTime(") => DataType::DateTime(match s[9..(s.len() - 1)].trim() {
				tz if tz.is_empty() => None,
				tz => Some(tz.replace('\'', "").parse()?),
			}),
			s if s.starts_with("DateTime64") => {
				let mut parts = s[11..(s.len() - 1)].splitn(2, ',');
				let precision = parts
					.next()
					.ok_or(Error::ParseTypeError(format!("Wrong DateTime64 type: {}", s)))?
					.parse::<u8>()?;
				let timezone = if let Some(zone) = parts.next() { Some(zone.replace('\'', "").parse()?) } else { None };
				DataType::DateTime64(precision, timezone)
			}
			s if s.starts_with("Nullable") => {
				let sub_str = &s[9..(s.len() - 1)];
				DataType::Nullable(Box::new(sub_str.parse()?))
			}
			s if s.starts_with("Array") => {
				let sub_str = &s[6..(s.len() - 1)];
				DataType::Array(Box::new(sub_str.parse()?))
			}
			s if s.starts_with("LowCardinality") => {
				let sub_str = &s[15..(s.len() - 1)];
				DataType::LowCardinality(Box::new(sub_str.parse()?))
			}
			s if s.starts_with("Map") => {
				let pair = parse_map(&s[4..(s.len() - 1)])?;
				DataType::Map(Box::new(pair.0), Box::new(pair.1))
			}
			"Json" => DataType::Json,
			s if s.starts_with("Tuple") => DataType::Tuple(parse_tuple(&s[6..(s.len() - 1)])?),
			s if s.starts_with("Enum8") => DataType::Enum8(parse_enum(&s[6..(s.len() - 1)])?),
			s if s.starts_with("Enum16") => DataType::Enum16(parse_enum(&s[7..(s.len() - 1)])?),
			s if s.starts_with("AggregateFunction") => {
				let pair = parse_agg_func(&s[18..(s.len() - 1)])?;
				DataType::AggregateFunction(pair.0, pair.1)
			}
			s if s.starts_with("SimpleAggregateFunction") => {
				let pair = parse_agg_func(&s[24..(s.len() - 1)])?;
				DataType::SimpleAggregateFunction(pair.0, pair.1)
			}
			s => Err(Error::ParseTypeError(format!("unsupported type: {}", s)))?,
		})
	}
}

/// Aggregate function type.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AggFunc {
	Any,
	AnyLast,
	Min,
	Max,
	Sum,
	SumWithOverflow,
	GroupBitAnd,
	GroupBitOr,
	GroupBitXor,
	GroupArrayArray,
	GroupUniqArrayArray,
	SumMap,
	MinMap,
	MaxMap,
	FuncName(String),
}

impl FromStr for AggFunc {
	type Err = crate::error::Error;

	fn from_str(name: &str) -> Result<Self> {
		match name.trim() {
			"any" => Ok(AggFunc::Any),
			"anyLast" => Ok(AggFunc::AnyLast),
			"min" => Ok(AggFunc::Min),
			"max" => Ok(AggFunc::Max),
			"sum" => Ok(AggFunc::Sum),
			"sumWithOverflow" => Ok(AggFunc::SumWithOverflow),
			"groupBitAnd" => Ok(AggFunc::GroupBitAnd),
			"groupBitOr" => Ok(AggFunc::GroupBitOr),
			"groupBitXor" => Ok(AggFunc::GroupBitXor),
			"groupArrayArray" => Ok(AggFunc::GroupArrayArray),
			"groupUniqArrayArray" => Ok(AggFunc::GroupUniqArrayArray),
			"sumMap" => Ok(AggFunc::SumMap),
			"minMap" => Ok(AggFunc::MinMap),
			"maxMap" => Ok(AggFunc::MaxMap),
			s => Ok(AggFunc::FuncName(s.to_owned())),
		}
	}
}

/// timezone of DateTime.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Tz(chrono_tz::Tz);

/// convert string to timezone.
impl FromStr for Tz {
	type Err = crate::error::Error;

	fn from_str(s: &str) -> Result<Self> {
		Ok(Tz(s.trim().parse().map_err(Error::ParseTypeError)?))
	}
}

#[cfg(test)]
mod tests {
	use crate::metadata::data_type::AggFunc;
	use crate::metadata::data_type::DataType;

	#[test]
	fn test_parse() {
		assert_eq!("FixedString(16)".parse::<DataType>().unwrap(), DataType::FixedString(16));
		assert_eq!("Decimal(9, 3)".parse::<DataType>().unwrap(), DataType::Decimal(9, 3));
		assert_eq!("Decimal32(3)".parse::<DataType>().unwrap(), DataType::Decimal32(3));
		assert_eq!("Decimal64(3)".parse::<DataType>().unwrap(), DataType::Decimal64(3));
		assert_eq!("Decimal128(3)".parse::<DataType>().unwrap(), DataType::Decimal128(3));
		assert_eq!("Decimal256(3)".parse::<DataType>().unwrap(), DataType::Decimal256(3));
		assert_eq!("DateTime".parse::<DataType>().unwrap(), DataType::DateTime(None));
		assert_eq!("DateTime()".parse::<DataType>().unwrap(), DataType::DateTime(None));
		assert_eq!(
			"DateTime('Asia/Istanbul')".parse::<DataType>().unwrap(),
			DataType::DateTime(Some("Asia/Istanbul".parse().unwrap()))
		);
		assert_eq!("DateTime64(3)".parse::<DataType>().unwrap(), DataType::DateTime64(3, None));
		assert_eq!(
			"DateTime64(3, 'Asia/Istanbul')".parse::<DataType>().unwrap(),
			DataType::DateTime64(3, Some("Asia/Istanbul".parse().unwrap()))
		);
		assert_eq!(
			"Nullable(DateTime64(3, 'Asia/Istanbul'))".parse::<DataType>().unwrap(),
			DataType::Nullable(DataType::DateTime64(3, Some("Asia/Istanbul".parse().unwrap())).into())
		);
		assert_eq!(
			"Array(Nullable(Int8))".parse::<DataType>().unwrap(),
			DataType::Array(Box::new(DataType::Nullable(DataType::Int8.into())))
		);
		assert_eq!(
			"LowCardinality(String)".parse::<DataType>().unwrap(),
			DataType::LowCardinality(DataType::String.into())
		);
		assert_eq!(
			"Map(String, Int32)".parse::<DataType>().unwrap(),
			DataType::Map(DataType::String.into(), DataType::Int32.into())
		);
		assert_eq!(
			"Map(LowCardinality(String), Int32)".parse::<DataType>().unwrap(),
			DataType::Map(DataType::LowCardinality(DataType::String.into()).into(), DataType::Int32.into())
		);
		assert_eq!(
			"Map(String, Array(Decimal64(18)))".parse::<DataType>().unwrap(),
			DataType::Map(DataType::String.into(), DataType::Array(DataType::Decimal64(18).into()).into())
		);
		assert_eq!(
			"Tuple(Array(String), s Map(String, Int64) , s2 Map(String, UInt64))".parse::<DataType>().unwrap(),
			DataType::Tuple(vec![
				("".to_owned(), DataType::Array(DataType::String.into())),
				("s".to_owned(), DataType::Map(DataType::String.into(), DataType::Int64.into())),
				("s2".to_owned(), DataType::Map(DataType::String.into(), DataType::UInt64.into())),
			])
		);
		assert_eq!(
			"Enum8('hello' = 1, 'world' = 2)".parse::<DataType>().unwrap(),
			DataType::Enum8(vec![("hello".to_owned(), 1), ("world".to_owned(), 2)])
		);
		assert_eq!(
			"Enum16('hello' = 1, 'world' = 2)".parse::<DataType>().unwrap(),
			DataType::Enum16(vec![("hello".to_owned(), 1), ("world".to_owned(), 2)])
		);
		assert_eq!(
			"AggregateFunction(groupBitmap, UInt64)".parse::<DataType>().unwrap(),
			DataType::AggregateFunction(AggFunc::FuncName("groupBitmap".to_owned()), vec![DataType::UInt64])
		);
		assert_eq!(
			"SimpleAggregateFunction(sum, Float64)".parse::<DataType>().unwrap(),
			DataType::SimpleAggregateFunction(AggFunc::Sum, vec![DataType::Float64])
		);
	}
}
