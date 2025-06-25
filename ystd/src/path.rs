//! Wrapper types around [camino]

use std::{borrow::Borrow, fs::Metadata};

use crate::{fs, io, prelude::*};

/// [camino::Utf8Path] newtype
#[repr(transparent)]
pub struct Utf8Path(pub camino::Utf8Path);
pub type YPath = Utf8Path;
pub type Path = YPath;

impl Utf8Path {
	pub fn new(path: &(impl AsRef<str> + ?Sized)) -> &Self {
		let path = camino::Utf8Path::new(path);
		unsafe { &*(path as *const camino::Utf8Path as *const Utf8Path) }
	}

	/// [camino::Utf8Path::as_str]
	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}

	/// [camino::Utf8PathBuf::to_path_buf]
	pub fn to_path_buf(&self) -> Utf8PathBuf {
		Utf8PathBuf(self.0.to_path_buf())
	}
}

impl Utf8Path {
	/// [camino::Utf8Path::join]
	#[inline]
	#[must_use]
	pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
		Utf8PathBuf(self.0.join(&path.as_ref().0))
	}
}

/// [fs] integrations
impl Utf8Path {
	/// [camino::Utf8Path::canonicalize_utf8]
	pub async fn canonicalize_utf8(&self) -> io::Result<Utf8PathBuf> {
		fs::canonicalize_utf8(self).await
	}

	/// [camino::Utf8Path::canonicalize_utf8]
	pub async fn canonicalize(&self) -> io::Result<Utf8PathBuf> {
		self.canonicalize_utf8().await
	}

	/// [camino::Utf8Path::metadata]
	pub async fn metadata(&self) -> io::Result<Metadata> {
		fs::metadata(self).await
	}

	/// [camino::Utf8Path::is_dir]
	pub async fn is_dir(&self) -> bool {
		let Ok(metadata) = self.metadata().await else {
			return false;
		};
		metadata.is_dir()
	}

	/// [camino::Utf8Path::is_file]
	pub async fn is_file(&self) -> bool {
		let Ok(metadata) = self.metadata().await else {
			return false;
		};
		metadata.is_file()
	}
}

// impl std::ops::Deref for YPath {
// 	type Target = camino::Utf8Path;

// 	fn deref(&self) -> &Self::Target {
// 		&self.0
// 	}
// }

impl AsRef<YPath> for YPath {
	fn as_ref(&self) -> &YPath {
		self
	}
}

impl AsRef<std::path::Path> for YPath {
	fn as_ref(&self) -> &std::path::Path {
		self.0.as_std_path()
	}
}

impl AsRef<Utf8Path> for &str {
	fn as_ref(&self) -> &Utf8Path {
		Utf8Path::new(self)
	}
}

impl ToOwned for YPath {
	type Owned = YPathBuf;

	fn to_owned(&self) -> Self::Owned {
		Utf8PathBuf(self.0.to_owned())
	}
}

impl std::fmt::Display for YPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::fmt::Debug for Utf8Path {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

/// [camino::Utf8PathBuf] newtype
pub struct Utf8PathBuf(pub camino::Utf8PathBuf);
pub type YPathBuf = Utf8PathBuf;
pub type PathBuf = YPathBuf;

impl std::ops::Deref for PathBuf {
	type Target = YPath;

	fn deref(&self) -> &Self::Target {
		Path::new(self.0.as_str())
	}
}

impl Borrow<YPath> for YPathBuf {
	fn borrow(&self) -> &YPath {
		Path::new(self.0.as_str())
	}
}

impl AsRef<YPath> for YPathBuf {
	fn as_ref(&self) -> &YPath {
		Path::new(self.0.as_str())
	}
}

impl From<camino::Utf8PathBuf> for PathBuf {
	fn from(path: camino::Utf8PathBuf) -> Self {
		Self(path)
	}
}

impl From<&YPath> for PathBuf {
	fn from(path: &YPath) -> Self {
		Self(path.0.into())
	}
}

impl TryFrom<std::path::PathBuf> for PathBuf {
	type Error = color_eyre::Report;

	fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
		value.try_into().wrap_err(
			"ystd::path Failed to convert from `std::path::PathBuf` to `ystd::path::PathBuf`",
		)
	}
}

impl std::fmt::Display for Utf8PathBuf {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}
