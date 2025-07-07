//! Wrapper types around [camino]

use std::{
	borrow::{Borrow, Cow},
	convert::Infallible,
	fmt,
	fs::Metadata,
	ops::Deref,
	path::{Path, PathBuf},
	rc::Rc,
	str::FromStr,
};

use crate::{error::ReportedError, fs, io, prelude::*};

/// [camino::Utf8PathBuf] newtype
#[derive(Clone)]
#[repr(transparent)]
pub struct Utf8PathBuf(pub camino::Utf8PathBuf);

impl Utf8PathBuf {
	#[must_use]
	pub fn new() -> Utf8PathBuf {
		Utf8PathBuf(camino::Utf8PathBuf::new())
	}

	#[must_use]
	pub fn as_path(&self) -> &Utf8Path {
		Utf8Path::new(self.0.as_path())
	}

	pub fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf> {
		camino::Utf8PathBuf::from_path_buf(path).map(Self)
	}

	#[must_use = "`self` will be dropped if the result is not used"]
	pub fn into_std_path_buf(self) -> PathBuf {
		self.into()
	}
}

impl Deref for Utf8PathBuf {
	type Target = Utf8Path;

	fn deref(&self) -> &Utf8Path {
		self.as_path()
	}
}

// /// *Requires Rust 1.68 or newer.*
// impl std::ops::DerefMut for Utf8PathBuf {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		unsafe { Utf8Path::assume_utf8_mut(&mut self.0) }
// 	}
// }

impl fmt::Debug for Utf8PathBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}

impl fmt::Display for Utf8PathBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

// impl<P: AsRef<Utf8Path>> Extend<P> for Utf8PathBuf {
// 	fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
// 		for path in iter {
// 			self.push(path);
// 		}
// 	}
// }

/// [camino::Utf8Path] newtype
#[repr(transparent)]
pub struct Utf8Path(pub camino::Utf8Path);

impl Utf8Path {
	pub fn new(path: &(impl AsRef<str> + ?Sized)) -> &Self {
		let path = camino::Utf8Path::new(path);
		// SAFETY: #[repr(transparent)]
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

	pub async fn read_dir(&self) -> io::Result<ReadDirUtf8> {
		self.read_dir_utf8().await
	}
}

// impl Clone for Box<Utf8Path> {
//     fn clone(&self) -> Self {
//         let boxed: Box<Path> = self.0.into();
//         let ptr = Box::into_raw(boxed) as *mut Utf8Path;
//         // SAFETY:
//         // * self is valid UTF-8
//         // * ptr was created by consuming a Box<Path> so it represents an rced pointer
//         // * Utf8Path is marked as #[repr(transparent)] so the conversion from *mut Path to
//         //   *mut Utf8Path is valid
//         unsafe { Box::from_raw(ptr) }
//     }
// }

impl fmt::Display for Utf8Path {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl fmt::Debug for Utf8Path {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
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

impl From<String> for Utf8PathBuf {
	fn from(string: String) -> Utf8PathBuf {
		Utf8PathBuf(string.into())
	}
}

impl FromStr for Utf8PathBuf {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Utf8PathBuf(s.into()))
	}
}

// ---
// From impls: borrowed -> borrowed
// ---

impl<'a> From<&'a str> for &'a Utf8Path {
	fn from(s: &'a str) -> &'a Utf8Path {
		Utf8Path::new(s)
	}
}

// ---
// From impls: borrowed -> owned
// ---

impl<T: ?Sized + AsRef<str>> From<&T> for Utf8PathBuf {
	fn from(s: &T) -> Utf8PathBuf {
		Utf8PathBuf::from(s.as_ref().to_owned())
	}
}

// impl<T: ?Sized + AsRef<str>> From<&T> for Box<Utf8Path> {
// 	fn from(s: &T) -> Box<Utf8Path> {
// 		Box::from(s)
// 		// Utf8PathBuf::from(s).into_boxed_path()
// 	}
// }

// impl From<&'_ Utf8Path> for Arc<Utf8Path> {
// 	fn from(path: &Utf8Path) -> Arc<Utf8Path> {
// 		let arc: Arc<Path> = Arc::from(AsRef::<Path>::as_ref(path));
// 		let ptr = Arc::into_raw(arc) as *const Utf8Path;
// 		// SAFETY:
// 		// * path is valid UTF-8
// 		// * ptr was created by consuming an Arc<Path> so it represents an arced pointer
// 		// * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
// 		//   *const Utf8Path is valid
// 		unsafe { Arc::from_raw(ptr) }
// 	}
// }

// impl From<&'_ Utf8Path> for Rc<Utf8Path> {
// 	fn from(path: &Utf8Path) -> Rc<Utf8Path> {
// 		let rc: Rc<Path> = Rc::from(AsRef::<Path>::as_ref(path));
// 		let ptr = Rc::into_raw(rc) as *const Utf8Path;
// 		// SAFETY:
// 		// * path is valid UTF-8
// 		// * ptr was created by consuming an Rc<Path> so it represents an rced pointer
// 		// * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
// 		//   *const Utf8Path is valid
// 		unsafe { Rc::from_raw(ptr) }
// 	}
// }

// impl<'a> From<&'a Utf8Path> for Cow<'a, Utf8Path> {
// 	fn from(path: &'a Utf8Path) -> Cow<'a, Utf8Path> {
// 		Cow::Borrowed(path)
// 	}
// }

// impl From<&'_ Utf8Path> for Box<Path> {
// 	fn from(path: &Utf8Path) -> Box<Path> {
// 		AsRef::<Path>::as_ref(path).into()
// 	}
// }

// impl From<&'_ Utf8Path> for Arc<std::path::Path> {
// 	fn from(path: &Utf8Path) -> Arc<Path> {
// 		AsRef::<Path>::as_ref(path).into()
// 	}
// }

// impl From<&'_ Utf8Path> for Rc<std::path::Path> {
// 	fn from(path: &Utf8Path) -> Rc<Path> {
// 		AsRef::<Path>::as_ref(path).into()
// 	}
// }

// impl<'a> From<&'a Utf8Path> for Cow<'a, std::path::Path> {
// 	fn from(path: &'a Utf8Path) -> Cow<'a, Path> {
// 		Cow::Borrowed(path.as_ref())
// 	}
// }

// ---
// From impls: owned -> owned
// ---

// impl From<Box<Utf8Path>> for Utf8PathBuf {
// 	fn from(path: Box<Utf8Path>) -> Utf8PathBuf {
// 		path.into_path_buf()
// 	}
// }

// impl From<Utf8PathBuf> for Box<Utf8Path> {
// 	fn from(path: Utf8PathBuf) -> Box<Utf8Path> {
// 		path.into_boxed_path()
// 	}
// }

// impl<'a> From<Cow<'a, Utf8Path>> for Utf8PathBuf {
// 	fn from(path: Cow<'a, Utf8Path>) -> Utf8PathBuf {
// 		path.into_owned()
// 	}
// }

impl From<Utf8PathBuf> for String {
	fn from(path: Utf8PathBuf) -> String {
		path.0.into_string()
	}
}

// impl From<Utf8PathBuf> for OsString {
// 	fn from(path: Utf8PathBuf) -> OsString {
// 		path.into_os_string()
// 	}
// }

// impl<'a> From<Utf8PathBuf> for Cow<'a, Utf8Path> {
// 	fn from(path: Utf8PathBuf) -> Cow<'a, Utf8Path> {
// 		Cow::Owned(path)
// 	}
// }

// impl From<Utf8PathBuf> for Arc<Utf8Path> {
// 	fn from(path: Utf8PathBuf) -> Arc<Utf8Path> {
// 		let arc: Arc<Path> = Arc::from(path.0);
// 		let ptr = Arc::into_raw(arc) as *const Utf8Path;
// 		// SAFETY:
// 		// * path is valid UTF-8
// 		// * ptr was created by consuming an Arc<Path> so it represents an arced pointer
// 		// * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
// 		//   *const Utf8Path is valid
// 		unsafe { Arc::from_raw(ptr) }
// 	}
// }

// impl From<Utf8PathBuf> for Rc<Utf8Path> {
// 	fn from(path: Utf8PathBuf) -> Rc<Utf8Path> {
// 		let rc: Rc<Path> = Rc::from(path.0);
// 		let ptr = Rc::into_raw(rc) as *const Utf8Path;
// 		// SAFETY:
// 		// * path is valid UTF-8
// 		// * ptr was created by consuming an Rc<Path> so it represents an rced pointer
// 		// * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
// 		//   *const Utf8Path is valid
// 		unsafe { Rc::from_raw(ptr) }
// 	}
// }

impl From<Utf8PathBuf> for PathBuf {
	fn from(path: Utf8PathBuf) -> PathBuf {
		path.0.into()
	}
}

// impl From<Utf8PathBuf> for Box<Path> {
// 	fn from(path: Utf8PathBuf) -> Box<Path> {
// 		PathBuf::from(path).into_boxed_path()
// 	}
// }

// impl From<Utf8PathBuf> for Arc<Path> {
// 	fn from(path: Utf8PathBuf) -> Arc<Path> {
// 		PathBuf::from(path).into()
// 	}
// }

// impl From<Utf8PathBuf> for Rc<Path> {
// 	fn from(path: Utf8PathBuf) -> Rc<Path> {
// 		PathBuf::from(path).into()
// 	}
// }

// impl<'a> From<Utf8PathBuf> for Cow<'a, Path> {
// 	fn from(path: Utf8PathBuf) -> Cow<'a, Path> {
// 		PathBuf::from(path).into()
// 	}
// }

// ---
// TryFrom impls
// ---

impl TryFrom<std::path::PathBuf> for Utf8PathBuf {
	type Error = FromPathBufError;

	fn try_from(path: std::path::PathBuf) -> Result<Utf8PathBuf, Self::Error> {
		camino::Utf8PathBuf::try_from(path)
			.map(Self)
			.map_err(ReportedError::new)
			.wrap_reported_err("ystd::path::Utf8PathBuf::from(PathBuf)")
	}
}

impl<'a> TryFrom<&'a Path> for &'a Utf8Path {
	type Error = FromPathError;

	fn try_from(path: &'a Path) -> Result<&'a Utf8Path, Self::Error> {
		<&camino::Utf8Path>::try_from(path)
			.map(Utf8Path::new)
			.map_err(ReportedError::new)
	}
}

pub type FromPathBufError = ReportedError<camino::FromPathBufError>;
pub type FromPathError = ReportedError<camino::FromPathError>;

// .. snip ..

// ---
// AsRef impls
// ---

impl AsRef<Utf8Path> for Utf8Path {
	#[inline]
	fn as_ref(&self) -> &Utf8Path {
		self
	}
}

impl AsRef<Utf8Path> for Utf8PathBuf {
	#[inline]
	fn as_ref(&self) -> &Utf8Path {
		self.as_path()
	}
}

impl AsRef<Utf8Path> for str {
	#[inline]
	fn as_ref(&self) -> &Utf8Path {
		Utf8Path::new(self)
	}
}

impl AsRef<Utf8Path> for String {
	#[inline]
	fn as_ref(&self) -> &Utf8Path {
		Utf8Path::new(self)
	}
}

impl AsRef<std::path::Path> for Utf8Path {
	#[inline]
	fn as_ref(&self) -> &std::path::Path {
		self.0.as_ref()
	}
}

impl AsRef<std::path::Path> for Utf8PathBuf {
	#[inline]
	fn as_ref(&self) -> &std::path::Path {
		self.0.as_ref()
	}
}

impl AsRef<str> for Utf8Path {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl AsRef<str> for Utf8PathBuf {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

// impl AsRef<OsStr> for Utf8Path {
// 	#[inline]
// 	fn as_ref(&self) -> &OsStr {
// 		self.as_os_str()
// 	}
// }

// impl AsRef<OsStr> for Utf8PathBuf {
// 	#[inline]
// 	fn as_ref(&self) -> &OsStr {
// 		self.as_os_str()
// 	}
// }

// ---
// Borrow and ToOwned
// ---

impl Borrow<Utf8Path> for Utf8PathBuf {
	#[inline]
	fn borrow(&self) -> &Utf8Path {
		self.as_path()
	}
}

impl ToOwned for Utf8Path {
	type Owned = Utf8PathBuf;

	#[inline]
	fn to_owned(&self) -> Utf8PathBuf {
		self.to_path_buf()
	}
}

// impl<P: AsRef<Utf8Path>> std::iter::FromIterator<P> for Utf8PathBuf {
// 	fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Utf8PathBuf {
// 		let mut buf = Utf8PathBuf::new();
// 		buf.extend(iter);
// 		buf
// 	}
// }
