pub type Result<T> = core::result::Result<T, crate::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Parse number error: {0}")]
	ParseIntError(#[from] core::num::ParseIntError),

	#[cfg(feature = "decimal")]
	#[error("Parse number to decimal error: {0}")]
	ParseDecimalError(#[from] rust_decimal::Error),

	#[cfg(feature = "json")]
	#[error("serde json error: {0}")]
	SerdeJsonError(#[from] serde_json::Error),

	#[error("Parse type error: {0}")]
	ParseTypeError(String),

	#[error(transparent)]
	AnyError(#[from] anyhow::Error),
}
