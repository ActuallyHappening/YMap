#[derive(Debug)]
pub struct Equation<LHS, RHS> {
    lhs: LHS,
    rhs: RHS,
}

/// Think a unique GUID that (lifetime) references into a `HashMap` which contains information about all relevant variables
pub trait Variable {}

impl<LHS, RHS> Equation<LHS, RHS> {
    pub fn new(lhs: LHS, rhs: RHS) -> Self {
        Self { lhs, rhs }
    }

    pub fn try_solve_for<V>(variable: V) -> Option<V::Domain> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct SingleValueVariable<ID> {
        pub id: ID,
    }

    #[test]
    fn solves_linear_equation() {
        let variable = SingleValueVariable {
            id: String::from("x"),
        };
        let linear_function =
            ymap_functions::polynomial::Linear::from_gradient_yint(2i64, 3, variable);
        let equation = Equation::new(
            linear_function,
            ymap_functions::identities::Constant::new(3i64),
        );
        equation.try_solve_for(variable);
    }
}
