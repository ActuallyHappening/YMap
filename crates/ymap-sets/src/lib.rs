pub type Error = errors::Error;
pub type Result<T> = core::result::Result<T, Error>;

pub trait Set<Item> {
    fn contains(&self, item: &Item) -> bool;
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

    pub struct SetElement<S: Set<I>, I> {
        item: I,
        set: S,
    }

    impl<S: Set<I>, I> SetElement<S, I> {
        pub unsafe fn new_unchecked(item: I, set: S) -> SetElement<S, I> {
            SetElement { item, set }
        }

        pub fn new(item: I, set: S) -> Result<SetElement<S, I>> {
            // SAFETY: We immediately check and error otherwise
            let this = unsafe { Self::new_unchecked(item, set) };
            if !this.check() {
                Err(ValueNotAnElement)?
            } else {
                Ok(this)
            }
        }

        pub fn check(&self) -> bool {
            self.set.contains(&self.item)
        }
    }
}

pub mod errors {
    #[derive(Debug, derive_more::From)]
    pub enum Error {
        SetElements(crate::elements::ValueNotAnElement),
    }
}
