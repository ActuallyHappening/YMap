pub use leptos::prelude::*;
pub use leptos_router::components::Redirect;
pub use utils::prelude::*;

pub use serde::{Deserialize, Serialize};

pub use crate::error::AppError;
pub type AppResult<T> = Result<T, AppError>;
pub use generic_err::GenericError;
