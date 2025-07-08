use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
#[error("{report}")]
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

	pub fn map_inner<U, F: FnOnce(Arc<T>) -> Option<Arc<U>>>(self, cb: F) -> ReportedError<U> {
		ReportedError {
			report: self.report,
			inner: self.inner.map(|inner| cb(inner)).flatten(),
		}
	}

	pub fn erase_inner<U>(self) -> ReportedError<U> {
		self.map_inner(|_| None)
	}

	pub fn get_inner(&self) -> Option<&T> {
		match &self.inner {
			None => None,
			Some(inner) => Some(inner.as_ref()),
		}
	}
}

#[extension(pub trait WrapReportedErr)]
impl<T, ET> Result<T, ReportedError<ET>>
where
	ET: Send + Sync + 'static,
{
	fn wrap_reported_err(self, msg: impl std::fmt::Display + Send + Sync + 'static) -> Self {
		self.map_err(|err| ReportedError {
			report: err.report.wrap_err(msg),
			inner: err.inner,
		})
	}
}
