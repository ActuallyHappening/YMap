pub type Error = errors::Error;
pub type Result<T> = core::result::Result<T, Error>;

pub trait Set<Item> {
    fn contains(&self, item: &Item) -> bool;
}

pub mod identities {
    use crate::Set;

    /// Set with only one valid element
    #[derive(Debug)]
    pub struct Singleton<I> {
        value: I,
    }

    impl<I: PartialEq> Set<I> for Singleton<I> {
        fn contains(&self, item: &I) -> bool {
            &self.value == item
        }
    }

    impl<I> Singleton<I> {
        pub fn new(value: I) -> Self {
            Self { value }
        }
    }

    #[derive(Debug)]
    pub struct NullSet;

    impl<I> Set<I> for NullSet {
        fn contains(&self, _item: &I) -> bool {
            false
        }
    }

    impl NullSet {
        pub fn new() -> Self {
            Self
        }
    }
}

pub mod sets {
    use crate::Set;

    /// All the real numbers
    #[derive(Debug)]
    pub struct Real;

    impl Set<f64> for Real {
        fn contains(&self, _item: &f64) -> bool {
            true
        }
    }
}

pub mod combinators {
    use crate::Set;

    #[derive(Debug)]
    pub struct ExceptZero<S> {
        inner: S,
    }

    impl<Item: num_traits::identities::Zero, S: Set<Item>> Set<Item> for ExceptZero<S> {
        fn contains(&self, item: &Item) -> bool {
            if item.is_zero() {
                false
            } else {
                self.inner.contains(item)
            }
        }
    }
}

pub mod elements {
    use crate::{Result, Set};

    #[derive(Debug)]
    pub struct ValueNotAnElement;

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
}

pub mod errors {
    #[derive(Debug, derive_more::From)]
    pub enum Error {
        SetElements(crate::elements::ValueNotAnElement),
    }
}
