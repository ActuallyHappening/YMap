#[derive(Debug)]
pub struct Equation<LHS, RHS> {
    lhs: LHS,
    rhs: RHS,
}

/// Think a unique GUID that (lifetime) references into a `HashMap` which contains information about all relevant variables
pub trait Variable {
    type Item;
    type Domain: ymap_sets::Set<Self::Item>;
}

impl<LHS, RHS> Equation<LHS, RHS> {
    pub fn new(lhs: LHS, rhs: RHS) -> Self {
        Self { lhs, rhs }
    }

    pub fn try_solve_for<V: Variable>(
        &self,
        variable: V,
    ) -> Option<ymap_sets::elements::SetElement<V::Item, V::Domain>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    pub struct SingleValueVariable<ID> {
        pub id: ID,
    }

    impl Variable for SingleValueVariable<String> {
        type Item = i64;
        type Domain = ymap_sets::sets::Real;
    }

    #[test]
    fn solves_linear_equation() {
        let variable = SingleValueVariable {
            id: String::from("x"),
        };
        let linear_function =
            ymap_functions::polynomial::Linear::from_gradient_yint(2i64, 3, variable.clone());
        let equation = Equation::new(
            linear_function,
            ymap_functions::identities::Constant::new(9i64),
        );
        equation.try_solve_for(variable);
    }
}
