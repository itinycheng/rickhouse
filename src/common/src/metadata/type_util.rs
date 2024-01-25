use std::num::ParseIntError;
use std::str::FromStr;

use crate::error::Error;
use crate::error::Result;
use crate::metadata::data_type::DataType;

use super::data_type::AggFunc;

/// parse map like Map(`String, Map(String, Int64)`)
pub(crate) fn parse_map(inner: &str) -> Result<(DataType, DataType)> {
	let vec = split_to_fields(inner);
	if vec.len() != 2 {
		return Err(Error::ParseTypeError(format!("Can't parse map type: Map({})", inner)));
	}

	Ok((vec[0].parse()?, vec[1].parse()?))
}

/// parse agg_func like AggregateFunction(`sum, Float64`) or SimpleAggregateFunction(`sum, Float64`)
pub(crate) fn parse_agg_func(inner: &str) -> Result<(AggFunc, Vec<DataType>)> {
	let vec = inner.split(',').collect::<Vec<_>>();
	let func = vec
		.first()
		.ok_or(Error::ParseTypeError(format!("No aggregate function name found, type: {}", inner)))?
		.trim()
		.parse::<AggFunc>()?;
	let data_types = vec.iter().skip(1).try_fold(Vec::new(), |mut vec, s| {
		vec.push(s.parse()?);
		Result::Ok(vec)
	})?;
	Ok((func, data_types))
}

/// parse tuple like Tuple(`Array(String), s Map(String, Int64), s Map(String, Int64)`)
pub(crate) fn parse_tuple(inner: &str) -> Result<Vec<(String, DataType)>> {
	split_to_fields(inner).iter().try_fold(Vec::new(), |mut vec, field| {
		vec.push(parse_tuple_field(field)?);
		Result::Ok(vec)
	})
}

/// parse str: 'hello'=1,world=2
pub(crate) fn parse_enum<T: FromStr<Err = ParseIntError>>(inner: &str) -> Result<Vec<(String, T)>> {
	let vec =
		inner.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).try_fold(Vec::new(), |mut vec, element| {
			let tuple =
				element.split_once('=').ok_or(Error::ParseTypeError(format!("invalid tuple element: {}", element)))?;
			vec.push((tuple.0.replace('\'', "").trim().to_ascii_lowercase(), tuple.1.trim().parse()?));
			Result::Ok(vec)
		})?;
	Ok(vec)
}

/// parse str: `Array(String)` or `s Map(String, Int64)`
fn parse_tuple_field(field: &str) -> Result<(String, DataType)> {
	let vec = field.trim().splitn(2, ' ').collect::<Vec<_>>();
	Ok(match vec.len() {
		1 => ("".to_owned(), vec[0].parse()?),
		2 => (vec[0].to_owned(), vec[1].parse()?),
		_ => unreachable!(),
	})
}

/// split type fields
fn split_to_fields(inner: &str) -> Vec<&str> {
	let mut fields: Vec<&str> = Vec::new();
	let mut remaining = inner;
	let mut skip = 0;
	while !remaining.is_empty() {
		if let Some(idx) = remaining.find_skip(',', skip) {
			let field_str = &remaining[..idx];
			if is_balanced_brackets(field_str) {
				fields.push(field_str);
				remaining = &remaining[(idx + 1)..];
				skip = 0;
			} else {
				skip += 1;
			}
		} else {
			fields.push(remaining);
			remaining = "";
		}
	}

	fields
}

/// check whether parentheses are balanced like `((()))`
fn is_balanced_brackets(s: &str) -> bool {
	let mut stack = Vec::new();
	for c in s.chars() {
		match c {
			'(' => stack.push(c),
			')' => {
				if stack.pop().is_none() {
					return false;
				}
			}
			_ => (),
		}
	}
	stack.is_empty()
}

pub(crate) trait FindSkip {
	fn find_skip(&self, p: char, skip_num: usize) -> Option<usize>;
}

impl FindSkip for str {
	fn find_skip(&self, p: char, skip_num: usize) -> Option<usize> {
		self.chars().enumerate().filter(|&(_, c)| c == p).nth(skip_num).map(|(idx, _)| idx)
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_find_skip() {
		let s = "ab,cde,dsd";
		println!("{:?}", s.find_skip(',', 1));

		let mut a = s.splitn(2, '3');
		println!("{:?}", a.next());
		println!("{:?}", a.next());
	}
}
