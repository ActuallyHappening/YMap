use crate::{error::ReportedError, prelude::*, time::TimeoutErr};

use std::time::Duration;
use tokio::sync::MutexGuard;

/// The only difference is that the lock will timeout after 1 second by default.
/// This is to avoid deadlocks
pub struct Mutex<T: ?Sized> {
	pub dur: std::time::Duration,
	pub mutex: tokio::sync::Mutex<T>,
}

// As long as T: Send, it's fine to send and share Mutex<T> between threads.
// If T was not Send, sending and sharing a Mutex<T> would be bad, since you can
// access T through Mutex<T>.
unsafe impl<T> Send for Mutex<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for Mutex<T> where T: ?Sized + Send {}

impl<T: ?Sized> Mutex<T> {
	const DEFAULT: Duration = Duration::from_secs(1);

	pub fn new(t: T) -> Self
	where
		T: Sized,
	{
		Mutex {
			dur: Self::DEFAULT,
			mutex: tokio::sync::Mutex::new(t),
		}
	}

	pub async fn lock(&self) -> Result<MutexGuard<'_, T>, TimeoutErr> {
		self.mutex
			.lock()
			.timeout(self.dur)
			.await
			.wrap_reported_err("ystd::sync::Mutex::lock timed out")
	}
}
