#![allow(dead_code)]

use crate::error::Result;
use crate::metadata::data_type::DataType;

pub type RowSpec = Vec<ColumnSpec>;

/// Column specification for data.
pub struct ColumnSpec {
	col_name: String,
	col_type: DataType,
	original_type: String,
}

impl ColumnSpec {
	pub fn try_new(col_name: String, original_type: String) -> Result<ColumnSpec> {
		Ok(ColumnSpec { col_name, col_type: original_type.parse()?, original_type })
	}

	pub fn is_nullable(&self) -> bool {
		self.col_type.is_nullable()
	}
}

#[cfg(test)]
mod tests {
	use crate::metadata::data_type::DataType;

	#[test]
	fn test() {
		println!("{}", matches!(&DataType::FixedString(2), DataType::FixedString(..)))
	}
}
