//! Wrapper types around [camino]

use std::{borrow::Borrow, convert::Infallible, fs::Metadata, str::FromStr};

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
	/// [camino::Utf8Path::starts_with]
	#[inline]
	#[must_use]
	pub fn starts_with(&self, base: impl AsRef<Utf8Path>) -> bool {
		self.0.starts_with(base.as_ref())
	}

	/// [camino::Utf8Path::extension]
	/// Wraps an `Option` with a better error message
	#[inline]
	#[must_use]
	pub fn extension(&self) -> color_eyre::Result<&str> {
		self.0
			.extension()
			.ok_or(eyre!("Path {} has no extension", self))
	}
}

impl Utf8Path {
	/// [camino::Utf8Path::join]
	#[inline]
	#[must_use]
	pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
		Utf8PathBuf(self.0.join(&path.as_ref().0))
	}

	/// [camino::Utf8Path::parent]
	#[inline]
	#[must_use]
	pub fn parent(&self) -> Option<&Utf8Path> {
		self.0.parent().map(Utf8Path::new)
	}

	/// [camino::Utf8Path::ancestors]
	#[inline]
	pub fn ancestors(&self) -> Utf8Ancestors<'_> {
		Utf8Ancestors(self.0.ancestors())
	}

	/// [camino::Utf8Path::file_name]
	#[inline]
	#[must_use]
	pub fn file_name(&self) -> color_eyre::Result<&str> {
		self.0
			.file_name()
			.ok_or(eyre!("ystd::path path {self} has no file_name"))
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

	/// Convenient way to provide a better error message
	pub async fn assert_dir(&self) -> color_eyre::Result<Metadata> {
		let metadata = self.metadata().await?;
		eyre_assert!(
			metadata.is_dir(),
			"ystd::path::Utf8Path::assert_dir({}): Path isn't a directory",
			self
		);
		Ok(metadata)
	}

	/// [camino::Utf8Path::is_file]
	pub async fn is_file(&self) -> bool {
		let Ok(metadata) = self.metadata().await else {
			return false;
		};
		metadata.is_file()
	}

	/// Convenient way to provide a better error message
	pub async fn assert_file(&self) -> color_eyre::Result<Metadata> {
		let metadata = self.metadata().await?;
		eyre_assert!(
			metadata.is_file(),
			"ystd::path::Utf8Path::assert_dir({}): Path isn't a file",
			self
		);
		Ok(metadata)
	}

	pub async fn file_type_exhaustive(&self) -> color_eyre::Result<FileTypeExhaustive> {
		let metadata = self.metadata().await?;
		if metadata.is_file() {
			Ok(FileTypeExhaustive::File)
		} else if metadata.is_dir() {
			Ok(FileTypeExhaustive::Dir)
		} else {
			Err(eyre!("Path {} isn't a file or directory", self)
				.wrap_err("ystd::path::file_type_exhaustive"))
		}
	}

	#[inline]
	pub async fn read_dir_utf8(&self) -> io::Result<ReadDirUtf8> {
		let path = self.0.to_owned();
		io::asyncify(move || {
			path.read_dir_utf8()
				.map(|inner| ReadDirUtf8 { inner })
				.map_err_std_io(|io| {
					Report::new(io).wrap_err(format!("ystd::path::Utf8Path::read_dir({})", path))
				})
		})
		.await
	}

	#[inline]
	pub async fn read_dir(&self) -> io::Result<ReadDirUtf8> {
		self.read_dir_utf8().await
	}
}

// impl std::ops::Deref for YPath {
// 	type Target = camino::Utf8Path;

// 	fn deref(&self) -> &Self::Target {
// 		&self.0
// 	}
// }

impl PartialEq for Utf8Path {
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}

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

pub enum FileTypeExhaustive {
	File,
	Dir,
}

/// [camino::Utf8Ancestors]
#[derive(Copy, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[repr(transparent)]
pub struct Utf8Ancestors<'a>(camino::Utf8Ancestors<'a>);

impl std::fmt::Debug for Utf8Ancestors<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.0, f)
	}
}

impl<'a> Iterator for Utf8Ancestors<'a> {
	type Item = &'a Utf8Path;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(Utf8Path::new)
	}
}

impl std::iter::FusedIterator for Utf8Ancestors<'_> {}

#[derive(Debug)]
pub struct ReadDirUtf8 {
	inner: camino::ReadDirUtf8,
}

impl Iterator for ReadDirUtf8 {
	type Item = io::Result<Utf8DirEntry>;

	fn next(&mut self) -> Option<io::Result<Utf8DirEntry>> {
		self.inner.next().map(|some| {
			some.map(Utf8DirEntry)
				.map_err_std_io(|io| Report::new(io).wrap_err("ystd::path::ReadDirUtf8::next"))
		})
	}
}

#[derive(Debug)]
pub struct Utf8DirEntry(camino::Utf8DirEntry);

impl Utf8DirEntry {
	#[inline]
	pub fn path(&self) -> &Utf8Path {
		Utf8Path::new(self.0.path())
	}

	#[inline]
	pub fn file_name(&self) -> &str {
		self.path().file_name().unwrap()
	}
}

/// [camino::Utf8PathBuf] newtype
#[derive(Clone)]
pub struct Utf8PathBuf(pub camino::Utf8PathBuf);
pub type YPathBuf = Utf8PathBuf;
pub type PathBuf = YPathBuf;

impl PartialEq for Utf8PathBuf {
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}

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

impl AsRef<Utf8Path> for Utf8PathBuf {
	fn as_ref(&self) -> &Utf8Path {
		Utf8Path::new(self.0.as_str())
	}
}

impl AsRef<std::path::Path> for Utf8PathBuf {
	fn as_ref(&self) -> &std::path::Path {
		self.0.as_ref()
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
		camino::Utf8PathBuf::try_from(value)
			.map(Utf8PathBuf)
			.wrap_err(
				"ystd::path Failed to convert from `std::path::PathBuf` to `ystd::path::PathBuf`",
			)
	}
}

impl FromStr for PathBuf {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Path::new(s).to_owned())
	}
}

impl std::fmt::Display for Utf8PathBuf {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::fmt::Debug for Utf8PathBuf {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}
