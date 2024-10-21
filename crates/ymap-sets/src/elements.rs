use crate::{Result, Set};

#[derive(Debug)]
pub struct ValueNotAnElement;

/// Upholds the invariant that [self.value] is [contained](Set::contain) within [self.domain]
pub struct SetElement<I, S: Set<I>> {
    value: I,
    domain: S,
}

impl<I, S: Set<I>> SetElement<I, S> {
    pub unsafe fn new_unchecked(value: I, domain: S) -> SetElement<I, S> {
        SetElement { value, domain }
    }

    pub fn new(value: I, domain: S) -> Result<SetElement<I, S>> {
        // SAFETY: We immediately check and error otherwise
        let this = unsafe { Self::new_unchecked(value, domain) };
        if !this.check() {
            Err(ValueNotAnElement)?
        } else {
            Ok(this)
        }
    }

    pub fn check(&self) -> bool {
        self.domain.contains(&self.value)
    }

    pub fn value(&self) -> &I {
        &self.value
    }

    pub fn domain(&self) -> &S {
        &self.domain
    }

    pub fn update_value(self, new_value: I) -> Result<Self> {
        Self::new(new_value, self.domain)
    }

    /// Tries to change the inner value, returning the old value if successful
    pub fn try_set_value(&mut self, new_value: I) -> Result<I> {
        if self.domain.contains(&new_value) {
            let old_value = std::mem::replace(&mut self.value, new_value);
            Ok(old_value)
        } else {
            Err(ValueNotAnElement)?
        }
    }
}
