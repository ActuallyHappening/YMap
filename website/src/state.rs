use ymap::auth::config::ProductionConfig;

use crate::prelude::*;

/// All necessary application state
#[derive(Debug, Clone)]
pub struct AppState {
	config: ProductionConfig,
	db: OnceCell<Surreal<Any>>,
}

impl AppState {
	/// Only async to force you to deal with db connection in async context
	pub async fn config(&self) -> &ProductionConfig {
		&self.config
	}

	pub async fn db(&self) -> Result<Surreal<Any>, AppError> {
		match self.db.get() {
			Some(db) => Ok(db.clone()),
			None => {
				let db = self.config().await.connect_ws().await?;
				self.db.set(db.clone()).expect("Was just None");
				Ok(db)
			}
		}
	}
}

/// Hook for retrieving [AppState].
pub fn app_state() -> AppState {
	use_context().expect("AppState not provided?")
}
