use std::str::FromStr;

use crate::error::Error;
use crate::error::Result;

use super::util::parse_enum;
use super::util::parse_tuple;

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
			s if s.starts_with("Decimal(") => {
				let sub_str = &s[8..(s.len() - 1)];
				let idx = sub_str.find(',').expect("Wrong decimal type!");
				DataType::Decimal(sub_str[..idx].parse()?, sub_str[(idx + 1)..].parse()?)
			}
			s if s.starts_with("Decimal32") => {
				let scale: u8 = s[10..(s.len() - 1)].parse()?;
				let precision = 10 - (f64::log10(scale as f64)).ceil() as u8;
				DataType::Decimal(precision, scale)
			}
			s if s.starts_with("Decimal64") => {
				let scale: u8 = s[10..(s.len() - 1)].parse()?;
				let precision = 19 - (f64::log10(scale as f64)).ceil() as u8;
				DataType::Decimal(precision, scale)
			}
			s if s.starts_with("Decimal128") => {
				let scale: u8 = s[11..(s.len() - 1)].parse()?;
				let precision = 39 - (f64::log10(scale as f64)).ceil() as u8;
				DataType::Decimal(precision, scale)
			}
			s if s.starts_with("Decimal256") => {
				let scale: u8 = s[11..(s.len() - 1)].parse()?;
				let precision = 77 - (f64::log10(scale as f64)).ceil() as u8;
				DataType::Decimal(precision, scale)
			}
			s if s.starts_with("DateTime(") || s == "DateTime" => DataType::DateTime(if s.len() > 9 {
				Some(s[9..(s.len() - 1)].replace('\'', "").parse()?)
			} else {
				None
			}),
			s if s.starts_with("DateTime64") => {
				let mut parts = s[11..(s.len() - 1)].splitn(2, ',');
				let precision = parts.next().expect("Wrong DateTime64 type").parse::<u8>()?;
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
				let sub_str = &s[4..(s.len() - 1)];
				let idx = sub_str.find(',').ok_or(Error::ParseTypeError(format!("Error map type: {}", s)))?;
				DataType::Map(Box::new(sub_str[..idx].parse()?), Box::new(sub_str[(idx + 1)..].parse()?))
			}
			"Json" => DataType::Json,
			s if s.starts_with("Tuple") => DataType::Tuple(parse_tuple(&s[6..(s.len() - 1)])?),
			s if s.starts_with("Enum8") => DataType::Enum8(parse_enum(&s[6..(s.len() - 1)])?),
			s if s.starts_with("Enum16") => DataType::Enum16(parse_enum(&s[7..(s.len() - 1)])?),
			s if s.starts_with("AggregateFunction") => {
				// AggregateFunction(sum, Float64)
				let vec = s[24..(s.len() - 1)].split(',').collect::<Vec<_>>();
				let agg_func = vec
					.first()
					.ok_or(Error::ParseTypeError(format!("No aggregate function name found, type: {}", s)))?
					.trim()
					.parse::<AggFunc>()?;
				let data_types = vec.iter().skip(1).try_fold(Vec::new(), |mut vec, s| {
					vec.push(s.parse()?);
					Result::Ok(vec)
				})?;
				DataType::AggregateFunction(agg_func, data_types)
			}
			s if s.starts_with("SimpleAggregateFunction") => {
				// SimpleAggregateFunction(sum, Float64)
				let vec = s[24..(s.len() - 1)].split(',').collect::<Vec<_>>();
				let agg_func = vec
					.first()
					.ok_or(Error::ParseTypeError(format!("No aggregate function name found, type: {}", s)))?
					.trim()
					.parse::<AggFunc>()?;
				let data_types = vec.iter().skip(1).try_fold(Vec::new(), |mut vec, s| {
					vec.push(s.parse()?);
					Result::Ok(vec)
				})?;
				DataType::SimpleAggregateFunction(agg_func, data_types)
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
			s => Ok(AggFunc::FuncName(s.to_string())),
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
		Ok(Tz(s.parse().map_err(Error::ParseTypeError)?))
	}
}

#[cfg(test)]
mod tests {
	use crate::metadata::data_type::DataType;

	#[test]
	fn test_parse() {
		println!("{:?}", "Map(String, Int32)".parse::<DataType>());
		println!("{:?}", "Array(Nullable(Int8))".parse::<DataType>());
		println!("{:?}", "Map(LowCardinality(String), Int32)".parse::<DataType>());
		println!("{:?}", "Map(String, Array(Decimal64(18)))".parse::<DataType>());
		println!("{:?}", "Tuple(Array(String), s Map(String, Int64) , s Map(String, Int64))".parse::<DataType>());
		println!("{:?}", "Enum8('hello' = 1, 'world' = 2)".parse::<DataType>());
		println!("{:?}", "Enum16('hello' = 1, 'world' = 2)".parse::<DataType>());
		println!("{:?}", "SimpleAggregateFunction(sum, Float64)".parse::<DataType>());
	}
}
