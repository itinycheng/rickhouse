pub type Result<T> = core::result::Result<T, crate::error::Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Parse number error: {0}")]
	ParseIntError(#[from] core::num::ParseIntError),

	#[error("Prompt error: {0}")]
	ParseTypeError(String),

	#[error(transparent)]
	AnyError(#[from] anyhow::Error),
}
