use crate::{error::ReportedError, prelude::*};

#[instrument(name = "ystd::which::which")]
pub fn which(binary_name: &'static str) -> Result<Utf8PathBuf, ReportedError<::which::Error>> {
	let path = ::which::which(binary_name)
		.map_err(ReportedError::new)
		.wrap_reported_err(format!("ystd::which::which({binary_name})"))?;
	let path = Utf8PathBuf::try_from(path).map_err(|err| {
		ReportedError::empty(
			Report::new(err)
				.wrap_err("ystd::which::which({binary_name})"),
		)
	})?;
	Ok(path)
}
