use serde::Deserialize;
use serde::Serialize;

use crate::Result;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Json(pub Vec<u8>);

impl Json {
	pub fn new(data: Vec<u8>) -> Self {
		Json(data)
	}

	/// TODO: little endian?
	pub fn try_to_serde_json(&self) -> Result<serde_json::Value> {
		Ok(serde_json::from_slice(self.0.as_slice())?)
	}
}
