use ymap_sets::{elements::SetElement, Set};

pub trait Function<InputDomain> {
    type OutputDomain;

    fn evaluate(&self, input: InputDomain) -> Self::OutputDomain;
}

pub mod identities {
    use ymap_sets::identities::Singleton;

    use crate::Function;

    #[derive(Debug)]
    pub struct Constant<I: Clone> {
        value: I,
    }

    impl<InputDomain, Item: Clone> Function<InputDomain> for Constant<Item> {
        type OutputDomain = Item;

        fn evaluate(&self, _input: InputDomain) -> Self::OutputDomain {
            self.value.clone()
        }
    }

    impl<I: Clone> Constant<I> {
        pub fn new(value: I) -> Self {
            Self { value }
        }
    }

    pub struct FnFunction<F> {
        func: F,
    }

    impl<F, I, O> Function<I> for FnFunction<F>
    where
        F: Fn(I) -> O,
    {
        type OutputDomain = F::Output;

        fn evaluate(&self, input: I) -> Self::OutputDomain {
            (self.func)(input)
        }
    }

    impl<F> FnFunction<F> {
        pub fn new(func: F) -> FnFunction<F> {
            FnFunction { func }
        }
    }
}

pub mod foundational {}

pub mod polynomial {
    #[derive(Debug)]
    pub struct Polynomial<I, V> {
        coefficients: Vec<I>,
        variable: V,
    }

    #[derive(Debug)]
    pub struct Quadratic<I, V> {
        /// TODO: NonZero invariant
        a: I,
        b: I,
        c: I,
        variable: V,
    }

    #[derive(Debug)]
    pub struct Linear<I, V> {
        m: I,
        c: I,
        variable: V,
    }

    impl<I, V> Linear<I, V> {
        pub fn from_gradient_yint(m: I, c: I, variable: V) -> Self {
            Self { m, c, variable }
        }
    }
}
