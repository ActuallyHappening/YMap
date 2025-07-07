use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
#[error("{report}")]
#[non_exhaustive]
pub struct ReportedError<T> {
	pub report: Report,
	#[source]
	pub inner: Option<Arc<T>>,
}

impl<T> ReportedError<T> {
	pub fn empty(report: Report) -> Self {
		Self {
			report,
			inner: None,
		}
	}

	pub fn new(err: T) -> Self
	where
		T: Send + Sync + core::error::Error + 'static,
	{
		let err = Arc::new(err);
		Self {
			report: Report::new(err.clone()),
			inner: Some(err),
		}
	}
}


