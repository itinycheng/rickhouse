macro_rules! impl_ip {
	($name: ident, $d_ty: ty, $ip_ty: ty) => {
		pub struct $name(pub $d_ty);

		impl $name {
			pub fn new(data: $d_ty) -> Self {
				Self(data)
			}
		}

		impl From<$name> for $ip_ty {
			fn from(value: $name) -> Self {
				let mut arr = value.0;
				arr.reverse();
				Self::from(arr)
			}
		}

		impl From<$name> for String {
			fn from(value: $name) -> Self {
				let addr: $ip_ty = value.into();
				addr.to_string()
			}
		}
	};
}

impl_ip!(IpV4, [u8; 4], std::net::Ipv4Addr);
impl_ip!(IpV6, [u8; 16], std::net::Ipv6Addr);

#[cfg(test)]
mod tests {
	use crate::values::ip::IpV4;
	use crate::values::ip::IpV6;

	#[test]
	fn test_ipv4() {
		let ipv4 = IpV4::new([1, 0, 0, 127]);
		let ip: String = ipv4.into();
		assert_eq!("127.0.0.1", &ip);
	}

	#[test]
	fn test_ipv6() {
		let mut arr = [0u8; 16];
		arr[0] = 1;
		let ipv6 = IpV6::new(arr);
		let ip: String = ipv6.into();
		assert_eq!("::1", &ip);
	}
}
