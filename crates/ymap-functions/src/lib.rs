pub trait Function {
    type Input;
    type Output;

    fn evaluate(input: Self::Input) -> Self::Output;
}
