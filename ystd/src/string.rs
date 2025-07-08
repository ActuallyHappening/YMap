use crate::prelude::*;

#[extension(pub trait StringTruncateLossyExt)]
impl String {
	/// Like [String::truncate] but doesn't panic.
	/// 
	/// Somebody please optimize this implementation
	fn truncated_lossy(mut self, new_len: usize) -> String {
		// SAFETY: We then copy basically the whole string confirming its all UTF-8
		unsafe { self.as_mut_vec() }.truncate(new_len);
		String::from_utf8_lossy(self.as_bytes()).into_owned()
	}
}
