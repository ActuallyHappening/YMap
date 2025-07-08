use crate::{error::ReportedError, prelude::*};
use base64::prelude::*;
pub use base64::*;

#[instrument(name = "ystd::base64::decode")]
pub fn decode<T: AsRef<[u8]> + core::fmt::Debug>(
	input: T,
) -> Result<Vec<u8>, ReportedError<base64::DecodeError>> {
	BASE64_STANDARD
		.decode(input)
		.map_err(ReportedError::new)
		.wrap_reported_err("Couldn't decode base64 encoded data")
}
