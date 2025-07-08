pub use hex::*;

use crate::error::{ReportedError, WrapReportedErr};
use crate::prelude::*;

#[instrument(name = "ystd::hex::decode")]
pub fn decode<T: AsRef<[u8]> + core::fmt::Debug>(
	input: T,
) -> Result<Vec<u8>, ReportedError<FromHexError>> {
	::hex::decode(input)
		.map_err(ReportedError::new)
		.wrap_reported_err("Couldn't decode hex encoded data")
}
