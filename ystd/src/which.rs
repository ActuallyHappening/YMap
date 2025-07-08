use crate::{error::ReportedError, io::asyncify, prelude::*};

#[instrument(name = "ystd::which::which")]
pub async fn which(
	binary_name: &'static str,
) -> Result<Utf8PathBuf, ReportedError<::which::Error>> {
	let path = move || {
		Ok(::which::which(binary_name)
			.map_err(ReportedError::new)
			.wrap_reported_err(format!("::which::which failed"))?)
	};
	let path = asyncify(path).await?;
	let path = Utf8PathBuf::try_from(path).map_err(|err| ReportedError::empty(Report::new(err)))?;
	Ok(path)
}
