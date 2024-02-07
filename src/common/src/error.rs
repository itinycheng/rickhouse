use std::fmt::Display;

pub type Result<T> = core::result::Result<T, crate::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Parse number error: {0}")]
	ParseIntError(#[from] core::num::ParseIntError),

	#[error("Try from number error: {0}")]
	TryFromIntError(#[from] core::num::TryFromIntError),

	#[cfg(feature = "decimal")]
	#[error("Parse number to decimal error: {0}")]
	ParseDecimalError(#[from] rust_decimal::Error),

	#[cfg(feature = "json")]
	#[error("Serde json error: {0}")]
	SerdeJsonError(#[from] serde_json::Error),

	#[error("Read leb128 error: {0}")]
	LEB128Error(#[from] leb128::read::Error),

	#[error("encoding utf-8 string error: {0}")]
	Utf8Error(#[from] std::str::Utf8Error),

	#[error("Serde deSer error: {0}")]
	SerdeError(String),

	#[error("Parse type error: {0}")]
	ParseTypeError(String),

	#[error("Not enough data")]
	NotEnoughData,

	#[error("Encoding error: {0}")]
	EncodingError(String),

	#[error("Deserialize from any not supported")]
	SerdeAnyNotSupported,

	#[error(transparent)]
	AnyError(#[from] anyhow::Error),
}

impl serde::de::Error for Error {
	fn custom<T: Display>(msg: T) -> Self {
		Error::SerdeError(msg.to_string())
	}
}
