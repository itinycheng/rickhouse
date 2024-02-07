use std::mem;
use std::u8;

use serde::de::IntoDeserializer;
use serde::Deserialize;
use serde::Deserializer;

use crate::metadata::DataType;
use crate::metadata::Metadata;
use crate::metadata::MetadataRef;
use crate::serde::buf::BufExp;
use crate::Error;

pub(crate) fn deserialize_header<B: BufExp>(buf: &mut B) -> crate::Result<Metadata> {
	let len = buf.read_u64_leb128()?;
	let names = (0..len).map(|_| buf.read_utf8_string()).collect::<Result<Vec<_>, _>>()?;
	let types = (0..len).map(|_| buf.read_utf8_string()?.parse::<DataType>()).collect::<Result<Vec<_>, _>>()?;
	Ok(names.into_iter().zip(types).collect())
}

pub(crate) fn deserialize_from<'de, T, B: BufExp>(data: B, metadata: &Metadata) -> crate::Result<T>
where
	T: Deserialize<'de>,
{
	let mut deserializer = RowBinaryDeserializer { metadata, data };
	T::deserialize(&mut deserializer)
}

pub(crate) struct RowBinaryDeserializer<'a, T> {
	metadata: MetadataRef<'a>,
	data: T,
}

impl<T: BufExp> RowBinaryDeserializer<'_, T> {
	fn ensure_size(&self, size: usize) -> crate::Result<()> {
		self.data.ensure_size(size)
	}

	fn read_size(&mut self) -> crate::Result<usize> {
		self.data.read_size()
	}

	fn read_arr<const N: usize>(&mut self) -> crate::Result<[u8; N]> {
		self.ensure_size(N)?;
		let mut arr = [0u8; N];
		self.data.copy_to_slice(&mut arr[..]);
		Ok(arr)
	}

	fn read_vec(&mut self, size: usize) -> crate::Result<Vec<u8>> {
		self.data.read_vec(size)
	}

	fn read_u64_leb128(&mut self) -> crate::Result<u64> {
		self.data.read_u64_leb128()
	}

	fn read_utf8_string(&mut self) -> crate::Result<String> {
		self.data.read_utf8_string()
	}
}

macro_rules! impl_deserialize_num {
	($ty: ty, $de_fn: ident, $visitor_fn: ident, $reader_fn: ident) => {
		fn $de_fn<V>(self, visitor: V) -> Result<V::Value, Self::Error>
		where
			V: serde::de::Visitor<'de>,
		{
			self.ensure_size(mem::size_of::<$ty>())?;
			visitor.$visitor_fn(self.data.$reader_fn())
		}
	};
}

impl<'de, T: BufExp> Deserializer<'de> for &mut RowBinaryDeserializer<'_, T> {
	type Error = crate::Error;

	fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::SerdeAnyNotSupported)
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.ensure_size(1)?;
		match self.data.get_u8() {
			0 => visitor.visit_bool(false),
			1 => visitor.visit_bool(true),
			n => Err(Error::EncodingError(format!("can't convert {} to bool value", n))),
		}
	}

	impl_deserialize_num!(i8, deserialize_i8, visit_i8, get_i8);
	impl_deserialize_num!(i16, deserialize_i16, visit_i16, get_i16_le);
	impl_deserialize_num!(i32, deserialize_i32, visit_i32, get_i32_le);
	impl_deserialize_num!(i64, deserialize_i64, visit_i64, get_i64_le);
	impl_deserialize_num!(i128, deserialize_i128, visit_i128, get_i128_le);
	impl_deserialize_num!(u8, deserialize_u8, visit_u8, get_u8);
	impl_deserialize_num!(u16, deserialize_u16, visit_u16, get_u16_le);
	impl_deserialize_num!(u32, deserialize_u32, visit_u32, get_u32_le);
	impl_deserialize_num!(u64, deserialize_u64, visit_u64, get_u64_le);
	impl_deserialize_num!(u128, deserialize_u128, visit_u128, get_u128_le);
	impl_deserialize_num!(f32, deserialize_f32, visit_f32, get_f32_le);
	impl_deserialize_num!(f64, deserialize_f64, visit_f64, get_f64_le);

	fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		unimplemented!("ClickHouse doesn't have char type")
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let size = self.read_size()?;
		let vec = self.read_vec(size)?;
		let str = core::str::from_utf8(&vec[..])?;
		visitor.visit_str(str)
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_string(self.read_utf8_string()?)
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let size = self.read_size()?;
		let vec = self.read_vec(size)?;
		visitor.visit_bytes(vec.as_slice())
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		let size = self.read_size()?;
		let vec = self.read_vec(size)?;
		visitor.visit_byte_buf(vec)
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		self.ensure_size(1)?;
		match self.data.get_u8() {
			0 => visitor.visit_some(self),
			1 => visitor.visit_none(),
			v => Err(Error::EncodingError(format!("invalid option symbol: {}", v))),
		}
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_struct<V>(
		self,
		name: &'static str,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		todo!()
	}

	fn deserialize_enum<V>(
		self,
		name: &'static str,
		variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_enum("".into_deserializer())
	}

	fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		unimplemented!("deserialize_identifier is not implemented")
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_unit()
	}

	fn is_human_readable(&self) -> bool {
		false
	}
}
