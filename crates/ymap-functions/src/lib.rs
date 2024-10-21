use ymap_sets::elements::SetElement;

pub trait Function {
    type Input: SetElement;
}
