pub struct Uuid(pub [u8; 16]);

impl Uuid {
	pub fn new(data: [u8; 16]) -> Uuid {
		Uuid(data)
	}
}

impl From<Uuid> for uuid::Uuid {
	fn from(value: Uuid) -> Self {
		Self::from_bytes_le(value.0)
	}
}

impl From<Uuid> for String {
	fn from(value: Uuid) -> Self {
		let uuid: uuid::Uuid = value.into();
		uuid.to_string()
	}
}
