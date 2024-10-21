pub trait Function {
    type Input;
    type Output;

    fn evaluate(input: Self::Input) -> Self::Output;
}

pub mod values {
    use ymap_sets::Set;

    pub struct SetElement<S: Set<I>, I> {
        set: S,
        item: I,
    }

    impl<S: Set<I>, I> SetElement<S, I> {}
}
