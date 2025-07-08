use crate::{error::ReportedError, prelude::*};

pub type TimeoutErr = ReportedError<tokio::time::error::Elapsed>;

#[extension(pub trait FutureTimeoutExt)]
impl<F> F
where
	F: IntoFuture,
{
	async fn timeout(
		self,
		duration: std::time::Duration,
	) -> Result<<F::IntoFuture as Future>::Output, TimeoutErr> {
		tokio::time::timeout(duration, self)
			.await
			.map_err(ReportedError::new)
			.wrap_reported_err("ystd::time::FutureTimeoutExt::timeout timed out")
	}
}
