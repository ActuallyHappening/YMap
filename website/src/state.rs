use crate::prelude::*;

/// All necessary application state
#[derive(Debug, Clone)]
pub struct AppState {
	config: ProductionConfig,
	db: std::cell::OnceCell<Surreal<Any>>,
}

impl AppState {
	pub fn local_storage_key(&self) -> String {
		String::from("db-jwt")
	}

	/// Only async to force you to deal with db connection in async context
	pub async fn config(&self) -> &ProductionConfig {
		&self.config
	}

	/// Makes sure the database session is normalized,
	/// i.e. pulls in from local storage
	pub async fn db(&self) -> Result<Surreal<Any>, AppError> {
		match self.db.get() {
			Some(db) => Ok(db.clone()),
			None => {
				let db = self.config().await.connect_ws().await?;
				self.db.set(db.clone()).expect("Was just None");

				// let (jwt_, _, _) =

				Ok(db)
			}
		}
	}
}

/// Hook for retrieving [AppState].
pub fn app_state() -> AppState {
	use_context().expect("AppState not provided?")
}

/// Call in root of application
pub fn provide_app_context() {
	let config = ProductionConfig::new();
	provide_context(AppState {
		config,
		db: std::cell::OnceCell::new(),
	})
}
