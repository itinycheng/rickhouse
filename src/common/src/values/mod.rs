#[cfg(feature = "bigint")]
pub mod bigint;
#[cfg(feature = "decimal")]
pub mod decimal;
pub mod ip;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "uuid")]
pub mod uuid;
pub mod value;

#[cfg(test)]
mod tests {

	#[test]
	fn test() {
		let bytes = b"Hello, \xF0\x9F\x8C\x8E"; // 包含无效的字节序列
		println!("{:?}", bytes);

		let s = String::from_utf8_lossy(bytes);
		println!("{}", s);

		let new_bytes = s.as_bytes();
		println!("{:?}", new_bytes);
	}
}
