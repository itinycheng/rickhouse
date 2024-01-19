use std::num::ParseIntError;
use std::str::FromStr;

use crate::error::Error;
use crate::error::Result;
use crate::metadata::data_type::DataType;

/// parse tuple like Tuple(`Array(String), s Map(String, Int64) , s Map(String, Int64)`)
pub(crate) fn parse_tuple(s: &str) -> Result<Vec<(String, DataType)>> {
	let mut tuples = Vec::new();
	let mut remaining = s;
	let mut skip = 0;
	while !remaining.is_empty() {
		if let Some(idx) = remaining.find_skip(',', skip) {
			let name_type_str = &remaining[..idx];
			if is_balanced_brackets(name_type_str) {
				tuples.push(parse_tuple_element(name_type_str)?);
				remaining = &remaining[(idx + 1)..];
				skip = 0;
			} else {
				skip += 1;
			}
		} else {
			tuples.push(parse_tuple_element(remaining)?);
			remaining = "";
		}
	}
	Ok(tuples)
}

/// parse str: 'hello'=1,world=2
pub(crate) fn parse_enum<T: FromStr<Err = ParseIntError>>(s: &str) -> Result<Vec<(String, T)>> {
	let vec = s.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).try_fold(Vec::new(), |mut vec, element| {
		let tuple =
			element.split_once('=').ok_or(Error::ParseTypeError(format!("invalid tuple element: {}", element)))?;
		vec.push((tuple.0.replace('\'', "").trim().to_ascii_lowercase(), tuple.1.trim().parse()?));
		Result::Ok(vec)
	})?;
	Ok(vec)
}

/// parse str: `Array(String)` or `s Map(String, Int64)`
fn parse_tuple_element(element: &str) -> Result<(String, DataType)> {
	let vec = element.trim().splitn(2, ' ').collect::<Vec<_>>();
	Ok(match vec.len() {
		1 => ("".to_string(), vec.first().unwrap().parse()?),
		2 => (vec.first().unwrap().to_string(), vec.get(1).unwrap().parse()?),
		_ => unreachable!(),
	})
}

/// check whether parentheses are balanced like `((()))`
pub(crate) fn is_balanced_brackets(s: &str) -> bool {
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
	fn find_skip(&self, p: char, num: usize) -> Option<usize>;
}

impl FindSkip for str {
	fn find_skip(&self, p: char, num: usize) -> Option<usize> {
		self.chars().enumerate().filter(|&(_, c)| c == p).nth(num).map(|(idx, _)| idx)
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_find_skip() {
		let s = "abcdedsd";
		println!("{:?}", s.find_skip('c', 2));
	}
}
