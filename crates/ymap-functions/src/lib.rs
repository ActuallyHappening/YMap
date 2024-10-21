use ymap_sets::{elements::SetElement, Set};

pub trait Function {
    type InputItem;
    type InputSet: Set<Self::InputItem>;

    type OutputItem;
    type OutputSet: Set<Self::OutputItem>;

    fn evaluate(
        input: SetElement<Self::InputItem, Self::InputSet>,
    ) -> SetElement<Self::OutputItem, Self::OutputSet>;
}

pub mod identities {
    use crate::Function;

    pub struct Constant<I> {
        value: I,
    }

    impl<I> Function for Constant<I> {}
}
