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

    impl Set<i64> for Real {
        fn contains(&self, _item: &i64) -> bool {
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

pub mod elements;

pub mod errors {
    #[derive(Debug, derive_more::From)]
    pub enum Error {
        SetElements(crate::elements::ValueNotAnElement),
    }
}
